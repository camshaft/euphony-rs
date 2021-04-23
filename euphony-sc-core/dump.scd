// generates a yml file with all of the ugens for the given server
//
// execute with `dump.scd`

var f;
f = File("dump.yml", "w");

~getArgDocs = {|class, method|
	var doc, args;
	doc = SCDoc.getMethodDoc(class.asString, '*' ++ method.asString);
	args = Dictionary.new;

	if(doc != nil, {
		var arguments = doc.children[1].findChild(\ARGUMENTS);
		if(arguments != nil, {
			arguments.children.do({|child|
				var name, info;
				name = child.text;
				// TODO handle other kinds of children node (e.g. table, links)
				if(child.children.size == 1, {
					var text = child.children[0].findChild(\TEXT);
					if(text != nil, {
						args.put(name, text.text);
					});
				});
			});
		});
	});

	args
};


f.write("ugens:\n");
UGen.subclasses.do({|sub|
	var rates, method;
	rates = SortedList.new(0);
	sub.class.methods.do({|meth|
		if([\ar, \kr, \ir].includes(meth.name), {
			rates.add(meth.name);
			method = meth;
		});
	});

	if(rates.isEmpty.not, {
		var argDocs, classDocs, json;
		argDocs = ~getArgDocs.(sub.name, method.name);
		f.write("- name: ");
		f.write(sub.name.asString);
		f.write("\n");
		f.write("  rates:\n");
		rates.do({|rate|
			f.write("    - ");
			f.write(rate);
			f.write("\n");
		});

		classDocs = SCDoc.documents.at("Classes/" ++ sub.name);
		f.write("  summary: |\n    ");
		f.write(classDocs.summary.asString);
		f.write("\n");

		f.write("  arguments:\n");
		method.keyValuePairsFromArgs.asAssociations.do({|pair|
			var doc = argDocs.at(pair.key.asString);
			f.write("  - name: ");
			f.write(pair.key);
			f.write("\n");
			if(pair.value != nil, {
				f.write("    default: ");
				f.write(pair.value.asString);
				f.write("\n");
			});
			if(doc != nil, {
				f.write("    doc: |\n      ");
				// TODO handle multi-line descriptions
				f.write(doc.asString);
				f.write("\n");
			});
		});
	});
});
f.close;
0.exit;
