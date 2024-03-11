// I (176122) esp_rmaker_param: Received params: {"Scenes":{"Scenes":[{"action":{"Light":{"Brightness":20,"Hue":221,"Power":true,"Saturation":41}},"id":"G5GW","info":"","name":"scene1","operation":"add"}]}}
// I (176162) esp_rmaker_param: Reporting params: {"Scenes":{"Scenes":[{"name":"scene1","id":"G5GW","action":{"Light":{"Brightness":20,"Hue":221,"Power":true,"Saturation":41}}}]}}
// I (205202) esp_rmaker_param: Received params: {"Scenes":{"Scenes":[{"id":"G5GW","operation":"activate"}]}}
// I (205212) esp_rmaker_param: Received params: {"Light":{"Brightness":20,"Hue":221,"Power":true,"Saturation":41}}
// I (205222) app_main: Received write request via : Scene Activate
// I (205222) app_main: Received value = true for Light - Power
// I (205232) esp_rmaker_param: Reporting params: {"Light":{"Power":true}}
// I (205242) app_main: Received write request via : Scene Activate
// I (205242) app_main: Received value = 20 for Light - Brightness
// I (205252) esp_rmaker_param: Reporting params: {"Light":{"Brightness":20}}
// I (205262) app_main: Received write request via : Scene Activate
// I (205262) app_main: Received value = 221 for Light - Hue
// I (205272) esp_rmaker_param: Reporting params: {"Light":{"Hue":221}}
// I (205282) app_main: Received write request via : Scene Activate
// I (205282) app_main: Received value = 41 for Light - Saturation
// I (205292) esp_rmaker_param: Reporting params: {"Light":{"Saturation":41}}
// W (220162) protocomm_httpd: Closing session with ID: 1149437204
// I (220812) esp_rmaker_node_config: Generated Node config of length 1959
// I (258662) esp_rmaker_param: Received params: {"Schedule":{"Schedules":[{"action":{"Light":{"Brightness":68,"Hue":79,"Power":true,"Saturation":78}},"id":"UYTS","name":"schedule1","operation":"add","triggers":[{"d":0,"m":1339}]}]}}
// I (258672) esp_schedule: Schedule UYTS will be active on: Thu Mar  7 22:19:00 2024 +0530[IST]. DST: No
// I (258682) esp_schedule: Starting a timer for 129 seconds for schedule UYTS
// I (258712) esp_rmaker_param: Reporting params: {"Schedule":{"Schedules":[{"name":"schedule1","id":"UYTS","enabled":true,"action":{"Light":{"Brightness":68,"Hue":79,"Power":true,"Saturation":78}},"triggers":[{"m":1339,"d":0,"ts":1709830140}]}]}}
// I (259422) esp_rmaker_param: Received params: {"Schedule":{"Schedules":[{"id":"UYTS","operation":"enable"}]}}
// I (259432) esp_schedule: Schedule UYTS will be active on: Thu Mar  7 22:19:00 2024 +0530[IST]. DST: No
// I (259432) esp_schedule: Starting a timer for 129 seconds for schedule UYTS
// I (259462) esp_rmaker_param: Reporting params: {"Schedule":{"Schedules":[{"name":"schedule1","id":"UYTS","enabled":true,"action":{"Light":{"Brightness":68,"Hue":79,"Power":true,"Saturation":78}},"triggers":[{"m":1339,"d":0,"ts":1709830140}]}]}}
// I (388442) esp_schedule: Schedule UYTS triggered
// I (388442) esp_rmaker_param: Received params: {"Light":{"Brightness":68,"Hue":79,"Power":true,"Saturation":78}}
// I (388442) app_main: Received write request via : Schedule
// I (388452) app_main: Received value = true for Light - Power
// I (388452) esp_rmaker_param: Reporting params: {"Light":{"Power":true}}
// I (388472) app_main: Received write request via : Schedule
// I (388472) app_main: Received value = 68 for Light - Brightness
// I (388472) esp_rmaker_param: Reporting params: {"Light":{"Brightness":68}}
// I (388482) app_main: Received write request via : Schedule
// I (388492) app_main: Received value = 79 for Light - Hue
// I (388492) esp_rmaker_param: Reporting params: {"Light":{"Hue":79}}
// I (388502) app_main: Received write request via : Schedule
// I (388512) app_main: Received value = 78 for Light - Saturation
// I (388512) esp_rmaker_param: Reporting params: {"Light":{"Saturation":78}}
// I (388582) esp_rmaker_param: Reporting params: {"Schedule":{"Schedules":[{"name":"schedule1","id":"UYTS","enabled":false,"action":{"Light":{"Brightness":68,"Hue":79,"Power":true,"Saturation":78}},"triggers":[{"m":1339,"d":0,"ts":0}]}]}}

use serde::{de::value, Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
#[allow(unused_imports)]
use log:: { info, warn, error, debug };

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SceneAction {
    //data: DeviceCbType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Scene {
    #[serde(default)]
    name: String,
    id: String,
    #[serde(default)]
    info: String,
    #[serde(default)]
    action: String,
    operation: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenePrivData {
    #[serde(rename = "Scenes")]
    scenes: Vec<Scene>,
    #[serde(skip)]
    totalscenes: i32,
}

#[allow(unused)]
impl ScenePrivData {
    fn get_scene_from_id(&self, id: &String) -> Option<&Scene> {
        for scene in &self.scenes {
            if &scene.id == id {
                debug!("Scene with id {} found in list for get.", scene.id);
                return Some(scene);
            }
        }
        debug!("Scene with id {} not found in list for get.", id);
        None
    }

    fn add_scene_to_list(&mut self, scene: Scene) {
        if self.get_scene_from_id(&scene.id) != None {
            info!("Scene with id {} already added to list. Not adding again.", scene.id);
            return;
        }
        debug!("Scene with id {} is being added to list.", scene.id);
        self.scenes.push(scene);
        self.totalscenes += 1;
        debug!("Scene added to list.");
    }

    fn remove_scene_from_list(&mut self, id: String) {
        if self.get_scene_from_id(&id) == None {
            info!("Scene with id {} already removed from list. Not removing again.", &id);
            return;
        }
        debug!("Scene with id {} is being removed from list.", &id);
        self.scenes.retain(|x| x.id != id);
        self.totalscenes -= 1;
        debug!("Scene removed from list.");
    }

    fn edit(&mut self, id:String, action:String) {
        if let Some(scene) = self.get_scene_from_id(&id) {
            debug!("Scene with id {} is being edited.", &id);
            let scene_: Scene = Scene{
                name: scene.name.to_string(),
                id: scene.id.to_string(),
                info: scene.info.to_string(),
                action,
                operation: String::new(),
            };
            self.remove_scene_from_list(id);
            self.add_scene_to_list(scene_);
            debug!("Scene has been edited.");
        }
        else {
            info!("Scene with id {} already removed from list. Cannot edit as a result.", &id);
        }
    }

    fn activate(&mut self, id: String) {
        // TODO:
    }

    fn deactivate(&mut self, id: String) {
        // TODO:
    }
}

pub(crate) static mut SCENES: ScenePrivData = ScenePrivData {
    scenes: Vec::new(),
    totalscenes: 0,
};

pub(crate) fn scenecb(params: HashMap<String, Value>) {
    if params.get("Scenes").is_some() {
        let scenes = params.get("Scenes").unwrap();
        match scenes.as_array() {
            Some(scenes_array) => {
                for scene in scenes_array.iter() {
                    match serde_json::from_str::<Value>(&scene.as_str().unwrap()) {
                        Ok(value) => {
                            let operation = value.get("operation").and_then(|v| v.as_str()).unwrap();
                            let id = value.get("id").and_then(|v| v.as_str()).unwrap();
                            let name = value.get("name").and_then(|v| v.as_str()).unwrap();
                            let action = value.get("action").and_then(|v| v.as_str()).unwrap();
                            let info = value.get("info").and_then(|v| v.as_str()).unwrap();

                            if operation == "add" {
                                //Rust demands an unsafe block for mutating static variable
                                unsafe { SCENES.add_scene_to_list(Scene {
                                    name: name.to_string(),
                                    id: id.to_string(),
                                    info: info.to_string(),
                                    action: action.to_string(),
                                    operation: operation.to_string(),
                                }) };
                            }
                            else if operation == "remove" {
                                //Rust demands an unsafe block for mutating static variable
                                unsafe { SCENES.remove_scene_from_list(id.to_string()) };
                            }
                            else if operation == "edit" {
                                //Rust demands an unsafe block for mutating static variable
                                unsafe { SCENES.edit(id.to_string(), action.to_string()) };
                            }
                            else if operation == "activate" {
                                //Rust demands an unsafe block for mutating static variable
                                unsafe { SCENES.activate(id.to_string()) };
                            }
                            else if operation == "deactivate" {
                                //Rust demands an unsafe block for mutating static variable
                                unsafe { SCENES.deactivate(id.to_string()) }
                            }
                            else {
                                info!("Unknown operation {} for scene {}", operation, id);
                            }
                        }
                        Err(e) => {
                            error!("Error parsing Scene: {:?}", e);
                        }
                    }
                }
            },
            _ => {},
        }
    }
}




