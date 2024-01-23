#![cfg(target_os = "espidf")]

use crate::mqtt::base::*;

impl From<&QoSLevel> for esp_idf_svc::mqtt::client::QoS {
    fn from(value: &QoSLevel) -> Self {
        match value {
            QoSLevel::AtMostOnce => esp_idf_svc::mqtt::client::QoS::AtMostOnce,
            QoSLevel::AtLeastOnce => esp_idf_svc::mqtt::client::QoS::AtLeastOnce,
            QoSLevel::ExactlyOnce => esp_idf_svc::mqtt::client::QoS::ExactlyOnce,
        }
    }
}

impl From<&esp_idf_svc::mqtt::client::Event<esp_idf_svc::mqtt::client::EspMqttMessage<'_>>>
    for MqttEvent
{
    fn from(
        value: &esp_idf_svc::mqtt::client::Event<esp_idf_svc::mqtt::client::EspMqttMessage<'_>>,
    ) -> Self {
        match value {
            esp_idf_svc::mqtt::client::Event::Connected(_) => MqttEvent::Connected,
            esp_idf_svc::mqtt::client::Event::Received(m) => MqttEvent::Publish(PublishMessage {
                topic: m.topic().unwrap().to_string(),
                payload: Vec::from(m.data()),
            }),
            esp_idf_svc::mqtt::client::Event::Disconnected => MqttEvent::Disconnected,
            esp_idf_svc::mqtt::client::Event::BeforeConnect => MqttEvent::BeforeConnect,
            esp_idf_svc::mqtt::client::Event::Published(_) => MqttEvent::Received,
            _ => Self::Other,
        }
    }
}

impl MqttClient<esp_idf_svc::mqtt::client::EspMqttClient<'_>> {
    pub fn new(
        config: &MqttConfiguration,
        callback: Box<dyn Fn(MqttEvent) + Send + Sync>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {

        let client_cert = std::ffi::CStr::from_bytes_with_nul(b"-----BEGIN CERTIFICATE-----\nMIIDPDCCAiSgAwIBAgIQcgX3ldk9mYKTMRMMsTdbkTANBgkqhkiG9w0BAQsFADBV\nMQswCQYDVQQGEwJJTjELMAkGA1UECBMCTUgxDTALBgNVBAcTBFB1bmUxEjAQBgNV\nBAoTCUVzcHJlc3NpZjEWMBQGA1UEAxMNRVNQLVJhaW5NYWtlcjAeFw0yNDAxMTMx\nNjU2MThaFw0yODAxMTMxNjU2MThaMBcxFTATBgNVBAMMDDU4Q0Y3OURBNEZEMDCC\nASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAKJEQgz0ROB5S0IEgTJVinMM\n8EMlQhjWvzdUm0sHqC3/Nqmxidi7bVf3Wk7aXN2XZ9LG2vTsr3m4t41PIdBFHhrp\ncJv0RHsheBvrOqKGLAF9gXiHyQGw9zG6LU8sID2lI7ZBrTzY9IH6qzx70cXQOGbb\nue8yfG/Qut5EzqWwm3Z8yDIPDmUsYMiI7jDiBBxJtPu1vWM+sD1K3ObcdOvUcDfF\n+oa5n07ay1eFKhLN7XQvgL51bk8hLIhSEZN9fAzxIvobA6fyxMCdGdy074N6Boen\nOGAmYadV7hw2JgkwKd9jS8P8jZZbuX5chw+5bgvw9uW5F0HG1DUhcYToUCkxoQsC\nAwEAAaNGMEQwDgYDVR0PAQH/BAQDAgeAMBMGA1UdJQQMMAoGCCsGAQUFBwMCMB0G\nA1UdDgQWBBS3ZCDgX0wv5R/J4yuBY8CIFvUg2DANBgkqhkiG9w0BAQsFAAOCAQEA\nCxhJc2Hc+cYhQXqZYxv5boriC4wqpfQn4SVU4BmLSK1einY6L5sP36P2jTPuOaGS\ngbWwTIEZhDERHE9w+huZCq2/P9BRrC+bPZbAElJVuFFS7aVQW6Z4midDBWaWcij5\nkJsbVdf3Zs/m9ev4uy+VL+jRQXtYgB+oSFBo9G2VpXXabjw9x6AsZqg26MTZx7BL\nP0UByTvU928uAUyEuOEIpc5GzI9see29Y2MwfcbeJhl+EW4wlJ8gCdNjEBIo4hmU\nJDReLSa7VVOZMTZLCBPWHej/4eocFxRNcBtZlfGE8u6v6dqZWEN1iv9rcX/LUqXE\njj2APrwOVqzIO0vJrJZkEQ==\n-----END CERTIFICATE-----\n-----BEGIN CERTIFICATE-----\nMIIDaDCCAlCgAwIBAgICBnUwDQYJKoZIhvcNAQELBQAwVTELMAkGA1UEBhMCSU4x\nCzAJBgNVBAgTAk1IMQ0wCwYDVQQHEwRQdW5lMRIwEAYDVQQKEwlFc3ByZXNzaWYx\nFjAUBgNVBAMTDUVTUC1SYWluTWFrZXIwHhcNMjAwNDI0MTMzNzIxWhcNMzAwNDI0\nMTMzNzIxWjBVMQswCQYDVQQGEwJJTjELMAkGA1UECBMCTUgxDTALBgNVBAcTBFB1\nbmUxEjAQBgNVBAoTCUVzcHJlc3NpZjEWMBQGA1UEAxMNRVNQLVJhaW5NYWtlcjCC\nASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAKHTyRirDV1QURE4wjpIWQyW\nVu7Qwjxvu+MkdOmec8gurN9DIPEQJOoa/pyfpuc1BceahjWUPdOxMwknKLc9dYs4\nyx/XkEOxRcY0OvtJg9Y2eAGgkuqKh+Z8DdFyAy2+VX48BaZxW4zf/a7cvGsQpffu\npNddVDJbLbK+Io0MT+tzcF9WM5ea4Hny4qBDeXXG6Uru4tnTTf/tnUqmHrVp95QT\nf9dMw+/98mEfpcQd35D9VxPwjTmZupx82AE/vvnu1m3vd1HzN/GEkdmHcvaMsNFh\nV6ucm2JTR9ocY+kBIOou3uTZmrZx6v6svKesDT3+Bmi8ncv2A71swCSjts6gDHMC\nAwEAAaNCMEAwDgYDVR0PAQH/BAQDAgKEMB0GA1UdJQQWMBQGCCsGAQUFBwMCBggr\nBgEFBQcDATAPBgNVHRMBAf8EBTADAQH/MA0GCSqGSIb3DQEBCwUAA4IBAQBnTjaZ\nIzMcL6aetdimuvWOfqrd1/Rvs3+HxFoZhU4utcV4ibg1O8MKaKejHtW3rDi+GLue\nykXDlo8UQdOEifng7WoQrKuRDuaF1dsaF4a80PBy5P4QHA9hensvkWldTZ2UqFrx\nO3sjrB+5chf4CoPEwEZ/ouKsMwFdgpFA3a7XTskwmuXQivXD8PGHXhPjLaRgAyZs\nO4psvoFW6QVXU2MRbNo2tiokQ2eVgW2t1vUdl0kjx5KMJQEdfY7ZmBFb+XL6goMD\nMIyP0BJg/V3WjuGYY3aWlkaob3TbBlePQkzAMtZlBtOjQsiwGBafIeOZcm2rN3Xv\nzy4NdNX/isyg/1C4\n-----END CERTIFICATE-----\x00").unwrap();
        let private_key = std::ffi::CStr::from_bytes_with_nul(b"-----BEGIN RSA PRIVATE KEY-----\nMIIEowIBAAKCAQEAokRCDPRE4HlLQgSBMlWKcwzwQyVCGNa/N1SbSweoLf82qbGJ\n2LttV/daTtpc3Zdn0sba9Oyvebi3jU8h0EUeGulwm/REeyF4G+s6ooYsAX2BeIfJ\nAbD3MbotTywgPaUjtkGtPNj0gfqrPHvRxdA4Ztu57zJ8b9C63kTOpbCbdnzIMg8O\nZSxgyIjuMOIEHEm0+7W9Yz6wPUrc5tx069RwN8X6hrmfTtrLV4UqEs3tdC+AvnVu\nTyEsiFIRk318DPEi+hsDp/LEwJ0Z3LTvg3oGh6c4YCZhp1XuHDYmCTAp32NLw/yN\nllu5flyHD7luC/D25bkXQcbUNSFxhOhQKTGhCwIDAQABAoIBAAnBB6Vms5M4x1TX\nF3sSmEl1MCYhIbmDgygMzm7yrWHicwM9WFduYNLGXCfcSXPKi6Oob3YEmkG7YFE+\nvf4agYZFnQ7K3qj2KJWpDLPDU/bc+ADqTKNs41caZWnaca+y8xQcG5FKS1xa2JtA\nqCn3a8SHFcSyqLdB+VGuGivsk1PKcO9lWvDzmODBsPmhUrNiba/qrjjX5tmTTq4g\nWpBpEIj4oE5JqTNnPByLLfFdoB9HgVUpISZjjpkr/a34ag8tjgAqNaX5lcXzJBoX\ngeqpFHbn9/QsQ8Cp/9WZ3otlGsb2qLq78mqmB0ePAq7em7oshAubZETjyuMbO7P3\nB8PffSECgYEAzJVUeaP4AK2GCw2DkixEo2onPTj34eUJj819iK7vAtkrRb9tjaV5\nJdW2AC2kyToQtC36qHjA7bQsZ2gQvaSh3KFVFId8jJmeQC4DZhOWbFgwfMwmZJ2f\nT0YPh0i1Z+wFdJQASYXVfkUkwfuchiqwAXEX7OewAJSBAl6+ATOiNmECgYEAywxR\narktPQY2VrB1uEzv2Nk9yGyuV/rdTvSWmco5vvxW+dxi4PPrxEIjvV7vJSSaSq9y\nRbj1L94mEHSGP2R34KJilmF2RKjFmNjLmC1hG8ttUb6MrmF9JKmG4f3xkU56B9WE\njsGS4AwA4Egc3fOgy1a1g1wydXlN4sXxZqbZdusCgYEAj6bjlC5AfClcD/LDSiZv\nY9esCd8wn5u1pRYDabB7/6ICMo2CHY3tjVWM4H8sUGfHRt1qPlXcEo0LCOkXqDIr\nvcJC/ZFNMWTErAgjNs/WNemO0nJ/GvNa480sJjA1wO0Hv54UvYuOJ4Xhk5xbghID\nWF/TDvR4r0+XbtSb0jgxVMECgYAjNUuY2etCPaWGeqqu/ohIbcMM0euZTeK6IidL\nG4nP2Cmswc5Te3hSW7WezKtjIWmvsaR6+otpdXfcOphcvasbxIybzuKXHTzGODF1\nfcjs7OVT21rBkh4FEXBWF5afv5/hY+DMcDxrpSkxus9mnSwwliR7Vq1ZOWOMAw7M\n4fmraQKBgBA9gQtPwQsGzxv9uFsyiV9YIOuRwzRCedcMgFEP/yeWdabc5zIXEPcq\nhMHzcuxJvyFSJVvFjmyAMGwkO15JSHCxxK5afDG90JvXwKwDcNyZFYbgEMATDEyS\njb7Hba4BJb3Nfiek+bTrxpU/QfywByNm2jbB0o/jaJcRbXnNvM+l\n-----END RSA PRIVATE KEY-----\x00").unwrap();
        let server_cert = std::ffi::CStr::from_bytes_with_nul(b"-----BEGIN CERTIFICATE-----\nMIIDQTCCAimgAwIBAgITBmyfz5m/jAo54vB4ikPmljZbyjANBgkqhkiG9w0BAQsF\nADA5MQswCQYDVQQGEwJVUzEPMA0GA1UEChMGQW1hem9uMRkwFwYDVQQDExBBbWF6\nb24gUm9vdCBDQSAxMB4XDTE1MDUyNjAwMDAwMFoXDTM4MDExNzAwMDAwMFowOTEL\nMAkGA1UEBhMCVVMxDzANBgNVBAoTBkFtYXpvbjEZMBcGA1UEAxMQQW1hem9uIFJv\nb3QgQ0EgMTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBALJ4gHHKeNXj\nca9HgFB0fW7Y14h29Jlo91ghYPl0hAEvrAIthtOgQ3pOsqTQNroBvo3bSMgHFzZM\n9O6II8c+6zf1tRn4SWiw3te5djgdYZ6k/oI2peVKVuRF4fn9tBb6dNqcmzU5L/qw\nIFAGbHrQgLKm+a/sRxmPUDgH3KKHOVj4utWp+UhnMJbulHheb4mjUcAwhmahRWa6\nVOujw5H5SNz/0egwLX0tdHA114gk957EWW67c4cX8jJGKLhD+rcdqsq08p8kDi1L\n93FcXmn/6pUCyziKrlA4b9v7LWIbxcceVOF34GfID5yHI9Y/QCB/IIDEgEw+OyQm\njgSubJrIqg0CAwEAAaNCMEAwDwYDVR0TAQH/BAUwAwEB/zAOBgNVHQ8BAf8EBAMC\nAYYwHQYDVR0OBBYEFIQYzIU07LwMlJQuCFmcx7IQTgoIMA0GCSqGSIb3DQEBCwUA\nA4IBAQCY8jdaQZChGsV2USggNiMOruYou6r4lK5IpDB/G/wkjUu0yKGX9rbxenDI\nU5PMCCjjmCXPI6T53iHTfIUJrU6adTrCC2qJeHZERxhlbI1Bjjt/msv0tadQ1wUs\nN+gDS63pYaACbvXy8MWy7Vu33PqUXHeeE6V/Uq2V8viTO96LXFvKWlJbYK8U90vv\no/ufQJVtMVT8QtPHRh8jrdkPSHCa2XV4cdFyQzR1bldZwgJcJmApzyMZFo6IQ6XU\n5MsI+yMRQ+hDKXJioaldXgjUkK642M4UwtBV8ob2xJNDd2ZhwLnoQdeXeGADbkpy\nrqXRfboQnoZsG4q5WTP468SQvvG5\n-----END CERTIFICATE-----\x00").unwrap();
        let mut options = esp_idf_svc::mqtt::client::MqttClientConfiguration::default();

        options.client_id = Some(config.clientid);

        options.server_certificate = Some(esp_idf_svc::tls::X509::pem(server_cert));
        options.client_certificate = Some(esp_idf_svc::tls::X509::pem(client_cert));
        options.private_key = Some(esp_idf_svc::tls::X509::pem(private_key));

        let mut conn_addr = if config.port == 8883 {
            "mqtts://"
        } else {
            "mqtt://"
        }
        .to_string();

        conn_addr += config.host;

        log::info!("connection string: {}", conn_addr);

        let client = esp_idf_svc::mqtt::client::EspMqttClient::new(
            conn_addr.as_str(),
            &options,
            move |req| match req {
                Ok(req) => callback(req.into()),
                Err(e) => log::error!("{:?}", e),
            },
        )
        .unwrap();
        // check whether thread is getting created 

        Ok(Self { client })
    }

    pub fn publish(&mut self, topic: &str, qos: &QoSLevel, payload: &str) {
        self.client
            .publish(topic, qos.into(), false, payload.as_bytes())
            .expect("unable to publish");
    }

    pub fn subscribe(&mut self, topic: &str, qos: &QoSLevel) {
        self.client
            .subscribe(topic, qos.into())
            .expect("unable to subscribe");
    }
}
