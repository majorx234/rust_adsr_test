fn main() {
    let ta: f32 = 0.1;
    let td: f32 = 0.2;
    let ts: f32 = 0.5;
    let tr: f32 = 0.3;
    let sustain_value: f32 = 0.3;
    let sample_size: usize = 96000;
    let frame_size: usize = 1024;

    let fmax_attack: f32 = ta * sample_size as f32;
    let fmax_decay: f32 = td * sample_size as f32;
    let fmax_sustain: f32 = ts * sample_size as f32;
    let fmax_release: f32 = tr * sample_size as f32;

    let max_attack: usize = fmax_attack as usize;
    let max_decay: usize = fmax_decay as usize;
    let max_sustain: usize = fmax_sustain as usize;
    let max_release: usize = fmax_release as usize;

    let frame_max_attack: usize = max_attack % frame_size;
    let frame_max_decay: usize = (max_attack + max_decay) % frame_size;
    let mut values: Vec<f32> = vec![0.0; 96000];

    let mut startpose: usize = 0;
    while (startpose < sample_size) {
        if startpose + frame_size < max_attack {
            for n in 0..frame_size {
                let k: usize = startpose + n;
                let s: f32 = ((k % max_attack) as f32) / fmax_attack;
                values[startpose + n] = s;
            }
        } else {
            if startpose < max_attack {
                //attack
                for n in 0..frame_max_attack {
                    let k: usize = startpose + n;
                    let s: f32 = ((k % max_attack) as f32) / fmax_attack;
                    values[startpose + n] = s;
                }
                if max_decay + frame_max_attack < frame_size {
                    //decay
                    for n in frame_max_attack..(frame_max_attack + max_decay) {
                        let k: usize = startpose + n;
                        let s: f32 = 1.0 - (0.7 * ((k % max_decay) as f32) / fmax_decay);
                        values[startpose + n] = s;
                    }
                    for n in (frame_max_attack + max_decay)..frame_size {
                        values[startpose + n] = sustain_value;
                    }
                } else {
                    //decay rest of frame
                    for n in frame_max_attack..frame_size {
                        let k: usize = startpose + n - max_attack;
                        let s: f32 = 1.0 - (0.7 * ((k % max_decay) as f32) / fmax_decay);
                        values[startpose + n] = s;
                    }
                }
            } else {
                // startpose > max_attack
                if startpose < (max_attack + max_decay - (frame_size - frame_max_decay)) {
                    //decay
                    for n in 0..frame_size {
                        let k: usize = startpose + n - max_attack;
                        let s: f32 = 1.0 - (0.7 * ((k % max_decay) as f32) / fmax_decay);
                        values[startpose + n] = s;
                    }
                    if (max_attack + max_decay) - startpose < frame_size {
                        let rest_sustain = max_attack + max_decay - startpose;
                        for n in 0..rest_sustain {
                            values[startpose + n] = sustain_value;
                        }
                    }
                } else {
                    //if startpose < (max_attack + max_decay + max_sustain) {
                    // WIP
                    // let rest_frame_size = (sample_size - startpose);
                    // println!("{}", startpose);
                    // println!("{}", rest_frame_size);
                    // let max_frame_size = frame_size.min(frame_size - rest_frame_size);
                    for n in 0..frame_size {
                        let index = startpose + n;
                        if index < sample_size {
                            values[index] = sustain_value;
                        }
                    }
                    //}
                    //else {

                    //}
                }
                // releasepart
            }
        }
        for n in 0..frame_size {
            let pose: usize = startpose + n;
            if pose < sample_size {
                println!("{}", values[pose]);
            }
        }
        startpose += frame_size;
    }
}
