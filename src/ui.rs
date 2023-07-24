use std::sync::mpsc;

use druid::widget::{Controller, Label, Flex, Switch, Slider, LensWrap};
use druid::{Widget, WidgetExt, Env, UpdateCtx};

use crate::models::{self, AppData};

struct VolumeUpdateCallback {
    tx_volume: mpsc::Sender<f32>,
}

impl Controller<f64, Slider> for VolumeUpdateCallback {
    fn update(&mut self, child: &mut Slider, ctx: &mut UpdateCtx<'_,'_>, old_data: &f64, data: &f64, env: &Env) {
        if *old_data != *data {
            println!("Volume changed to {}", 2.0 * (*data));
            let _ = self.tx_volume.send(*data as f32);
        }

        child.update(ctx, old_data, data, env)
    }
}

struct DenoiseUpdateCallback {
    tx_denoise: mpsc::Sender<bool>
}

impl Controller<bool, Switch> for DenoiseUpdateCallback {
    fn update(&mut self, child: &mut Switch, ctx: &mut UpdateCtx, old_data: &bool, data: &bool, env: &Env) {
        if *old_data != *data {
            println!("Toggle denoise changed to {}", data);
            let _ = self.tx_denoise.send(*data);
        }

        child.update(ctx, old_data, data, env)
    }
}

pub fn ui_builder(tx_volume: mpsc::Sender<f32>, tx_denoise: mpsc::Sender<bool>) -> impl Widget<models::AppData> {
    let denoise_label = Label::new("Denoise").align_left();
    let denoise_switch = LensWrap::new(Switch::new().controller(DenoiseUpdateCallback {tx_denoise: tx_denoise}), AppData::denoise).align_right();
    let denoise_row = Flex::row().with_child(denoise_label).with_child(denoise_switch).align_left().padding(5.0);

    let volume_label = Label::new("Volume").align_left();
    let volume_slider = LensWrap::new(Slider::new().controller(VolumeUpdateCallback {tx_volume: tx_volume}), AppData::volume).align_right();
    let volume_row = Flex::row().with_child(volume_label).with_child(volume_slider).padding(5.0).align_left();

    Flex::column().with_child(volume_row).with_child(denoise_row).padding(5.0)
}
