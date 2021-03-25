import { default as React, Dispatch, SetStateAction } from "react";
import CssBaseline from "@material-ui/core/CssBaseline";
import { Bar } from "./Bar";
import { Scheduler } from "./Scheduler";
import {
  useMIDIConnectionManager,
  useMIDI,
  Connection,
  Input,
  Output,
} from "@react-midi/hooks";

type ConnectionManagerTuple = [
  Connection | null,
  Dispatch<SetStateAction<string>>
];

function App() {
  const { inputs, outputs } = useMIDI();
  const [input, setInput] = useMIDIConnectionManager(
    inputs
  ) as ConnectionManagerTuple;
  const [output, setOutput] = useMIDIConnectionManager(
    outputs
  ) as ConnectionManagerTuple;

  return (
    <>
      <CssBaseline />
      <Bar
        input={input ? input.id : ""}
        setInput={setInput}
        output={output ? output.id : ""}
        setOutput={setOutput}
      />
      <Scheduler input={input as Input} output={output as Output} />
    </>
  );
}

export default App;
