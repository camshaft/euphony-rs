try({
	i = "IN_DIR".getenv;
	if(i.isNil, {
		i = "synths";
	});
	i = PathName.new(i.resolveRelative);

	"cargo:rerun-if-changed=".post;
	i.fullPath.postln;

	o = "OUT_DIR".getenv;
	if(o.isNil, {
		o = "target/synths";
	});
	o = PathName.new(o.resolveRelative);
	o.fullPath.mkdir;

	~compileSynth = {|path|
		var synthdef, out;
		"compiling: ".post;
		path.postln;
		synthdef = path.load;
		synthdef.writeDefFile(o.fullPath, true);
	};

	if(i.isFolder, {
		i.deepFiles.do({|file|
			if(file.extension == "scd", {
				~compileSynth.(file.fullPath);
			});
		});
	});

	0.exit;
}, {|err|
	err.reportError.postln;
	1.exit;
});