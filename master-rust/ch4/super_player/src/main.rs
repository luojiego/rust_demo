mod media; // main.rs 同级包的处理方式
use media::Playable; // 这样处理完之后，还要添加 use 引入 

mod inheritance;
use inheritance::*; // TeslaRoadcast;
// 我去同样是个炸弹啊，如果不将 Car 和 Vehicle 引入的话，代码是无法编译

struct Audio(String);
struct Video(String);


impl Playable for Audio {
    fn play(&self) {
        println!("Now playing audio: {}", self.0);
    }
}

impl Playable for Video {
    fn play(&self) {
        println!("Now playing video: {}", self.0);
    }
}

fn main() {
    println!("Super player");
    let audio = Audio("ambient_music.mp3".to_string());
    let video = Video("big_buck_bunny.mkv".to_string());
    audio.play();
    video.play();

    let my_roadster = TeslaRoadcast::new("Tesla Roadster I", 2020);
    println!("{} is priced at ${}", my_roadster.model(), my_roadster.get_price());
}
