SynthDef(\simple_env, {|out = 0, freq = 440|
  var env, signal;
  env = Env([0, 1, 0.5, 1, 0], [0.01, 0.5, 0.02, 0.5]);
  signal = SinOsc.ar(freq) * EnvGen.kr(env, doneAction: Done.freeSelf);
  Out.ar(out, signal);
}).add;
