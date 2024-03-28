// I (176122) esp_rmaker_param: Received params: {"Scenes":{"Scenes":[{"action":{"Light":{"Brightness":20,"Hue":221,"Power":true,"Saturation":41}, "LED":{"Power":true}},"id":"G5GW","info":"","name":"scene1","operation":"add"}]}}
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

use serde::{ Deserialize, Serialize };
use serde_json::Value;
use std::collections::HashMap;
#[allow(unused_imports)]
use log:: { info, warn, error, debug };

use crate::node::Node;
use crate::Rainmaker;

const MAX_SCENES: usize = 5;
// ToDo: Implement a way to store the values in the NVS for saving the values even if the device turns off

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Scene {
    #[serde(default)]
    pub(crate) name: String,
    pub(crate) id: String,
    #[serde(default, skip_serializing)]
    pub(crate) info: String,
    #[serde(default)]
    pub(crate) action: HashMap<String, HashMap<String, Value>>,
    #[serde(skip_serializing)]
    pub(crate) operation: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenePrivData {
    #[serde(rename = "Scenes")]
    pub scenes: Vec<Scene>,
    #[serde(skip)]
    pub totalscenes: usize,
}

#[allow(unused)]
impl ScenePrivData {
    fn get_scene_from_id(&self, id: &String) -> Option<&Scene> {
        for scene in &self.scenes {
            if &scene.id == id {
                debug!("Scene with id {} found in list for get.", &id);
                return Some(scene);
            }
        }
        debug!("Scene with id {} not found in list for get.", &id);
        None
    }

    pub fn add_scene_to_list(&mut self, scene: Scene) {
        if self.get_scene_from_id(&scene.id) != None {
            info!("Scene with id {} already added to list. Not adding again.", scene.id);
            return;
        }
        if self.totalscenes >= MAX_SCENES {
            info!("Maximum number of scenes reached. Not adding any more.");
            return;
        }
        debug!("Scene with id {} is being added to list.", scene.id);
        self.scenes[self.totalscenes] = scene;
        self.totalscenes += 1;
        debug!("Scene added to list.");
        //self.report_params()
    }

    pub fn remove_scene_from_list(&mut self, id: String) {
        if self.get_scene_from_id(&id) == None {
            info!("Scene with id {} already removed from list. Not removing again.", &id);
            return;
        }
        debug!("Scene with id {} is being removed from list.", &id);
        self.scenes.retain(|x| x.id != id);
        self.totalscenes -= 1;
        debug!("Scene removed from list.");
    }

    pub fn edit(&mut self, id:String, action: HashMap<String, HashMap<String, Value>>) {
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

    pub fn activate(&mut self, id: String, node: &Node) {
        // TODO:
        if let Some(scene) = self.get_scene_from_id(&id) {
            debug!("Scene with id {} is being activated.", &id);
            for (device, param) in scene.action.iter() {
                node.exeute_device_callback(device, param);
            }
        }
    }

    pub fn deactivate(&mut self, id: String, node: &Node) {
        // TODO: Save initial values and then save it while activating thus need more space
    }
}

// pub(crate) static mut SCENES: ScenePrivData = ScenePrivData {
//     scenes: Vec::new(),
//     totalscenes: 0,
// };





