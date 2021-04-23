SynthDef(\euphony_play_buf_2, {| out = 0, buf = 0, rate = 1, amp = 1 |
    Out.ar(out,
        PlayBuf.ar(1, buf, BufRateScale.kr(buf) * rate) * amp
    )
}).add;
