// example from https://depts.washington.edu/dxscdoc/Help/Guides/Non-Realtime-Synthesis.html

s = SynthDef(\NRTsine, { |out, freq = 440|
  Out.ar(out, SinOsc.ar(freq, 0, 0.2).dup)
}).asBytes;

n = File("nrtsine.synthdef", "w");
n.write(s);
n.close;

f = File("simple-sin.osc", "w");

c = [0.0,
  [\d_recv, s],
  [\s_new, \NRTsine, 1000, 1, 0]
].asRawOSC;
f.write(c.size); // each bundle is preceded by a 32 bit size.
f.write(c); // write the bundle data.

// stop sine oscillator at 3.0 seconds.
c = [3.0, [\n_free, 1000]].asRawOSC;
f.write(c.size);
f.write(c);

f.close;

0.exit;
