// For RNG Generation
use soloud::{AudioExt, LoadExt, Soloud, Wav};
use std::{
    process::Stdio,
    time::{SystemTime, UNIX_EPOCH},
};
use tiny_rng::{Rand, Rng};

macro_rules! voicelines {
    ([failure] $($failure:ident, $num:expr);*; [success] $($success:ident, $num2:expr);*) => {
        $(
            #[inline]
            fn $failure(wav: &mut Wav) {
                wav.load_mem(include_bytes!(concat!("../voicelines/failure/", $num, ".wav"))).unwrap();
            }
        )*

          $(  #[inline]
            fn $success(wav: &mut Wav) {
                wav.load_mem(include_bytes!(concat!("../voicelines/success/", $num2, ".wav"))).unwrap();
            }
        )*
    };
}

voicelines! {
    [failure]
    voiceline_1, 1;
    voiceline_2, 2;
    voiceline_3, 3;
    voiceline_4, 4;
    voiceline_5, 5;
    voiceline_6, 6;
    voiceline_7, 7;
    voiceline_8, 8;
    voiceline_9, 9;
    voiceline_10, 10;
    voiceline_11, 11;
    voiceline_12, 12;
    voiceline_13, 13;
    voiceline_14, 14;
    voiceline_15, 15;
    voiceline_16, 16;
    voiceline_17, 17;
    voiceline_18, 18;
    voiceline_19, 19;

    [success]
    voiceline_20, 20;
    voiceline_21, 21;
    voiceline_22, 22

}

fn main() -> Result<(), i32> {
    let mut rng = Rng::from_seed(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time has gone backwards, no more cake is available :(")
            .as_millis() as u64,
    );

    let args: Vec<String> = std::env::args().collect();

    if std::process::Command::new("cargo")
        .args(&args[1..])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Couldn't spawn command")
        .wait()
        .expect("Couldn't wait for command?")
        .success()
    {
        if rng.rand_u32() % 11 == 0 {
            let voiceline = rng.rand_range_u32(20, 23);
            let sl = Soloud::default().unwrap();
            let mut wav = Wav::default();
            match voiceline {
                20 => voiceline_20(&mut wav),
                21 => voiceline_21(&mut wav),
                22 => voiceline_22(&mut wav),
                _ => return Ok(()),
            }
            sl.play(&wav);
            while sl.voice_count() > 0 {
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
        } else {
            let voiceline = rng.rand_range_u32(1, 20);
            let sl = Soloud::default().unwrap();
            let mut wav = Wav::default();
            match voiceline {
                1 => voiceline_1(&mut wav),
                2 => voiceline_2(&mut wav),
                3 => voiceline_3(&mut wav),
                4 => voiceline_4(&mut wav),
                5 => voiceline_5(&mut wav),
                6 => voiceline_6(&mut wav),
                7 => voiceline_7(&mut wav),
                8 => voiceline_8(&mut wav),
                9 => voiceline_9(&mut wav),
                10 => voiceline_10(&mut wav),
                11 => voiceline_11(&mut wav),
                12 => voiceline_12(&mut wav),
                13 => voiceline_13(&mut wav),
                14 => voiceline_14(&mut wav),
                15 => voiceline_15(&mut wav),
                16 => voiceline_16(&mut wav),
                17 => voiceline_17(&mut wav),
                18 => voiceline_18(&mut wav),
                19 => voiceline_19(&mut wav),
                _ => return Ok(()),
            }
            sl.play(&wav);
            while sl.voice_count() > 0 {
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
        }
    }

    Ok(())
}
