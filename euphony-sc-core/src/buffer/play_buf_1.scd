SynthDef(\euphony_play_buf_1, {| out = 0, buf = 0, rate = 1, pan = 0, amp = 0.5 |
    Out.ar(out,
        Pan2.ar(
            PlayBuf.ar(1, buf, BufRateScale.kr(buf) * rate) * amp,
            pan
        )
    )
}).add;
