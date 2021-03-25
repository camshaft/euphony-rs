simple-sin.aif: simple-sin.osc
	@scsynth -N $< _ $@ 48000 AIFF int24 -o 2

simple-sin.osc: simple-sin.scd
	@sclang $<
