import {
    useMIDIMessage,
    useMIDI,
    Connection,
    Input,
    Output,
    MIDIMessage,
} from "@react-midi/hooks";
import { default as React, useEffect, useState } from "react";
import Button from "@material-ui/core/Button";
import { useNative } from "./native";

function useConnectInput(
    input: Input | null,
    onMessage: (msg: MIDIMessage) => void,
    watchers: any[]
) {
    useEffect(() => {
        if (input == null) return () => {};
        input.onmidimessage = onMessage;
        return () => (input.onmidimessage = () => {});
    }, [input].concat(watchers));
}

export function Scheduler({ input, output }: { input: Input; output: Output }) {
    const [native, error] = useNative();
    const [mode, setMode] = useState(0);
    const [tonic, setTonic] = useState(0);

    useConnectInput(
        input,
        (msg: MIDIMessage) => {
            if (!(native && native.process) || !output) return;
            const outMsg = new Uint8Array(32);
            const len = native.process(
                new Uint8Array(msg.data),
                outMsg,
                mode,
                tonic
            );
            if (!len) return;
            output.send((outMsg.slice(0, len) as unknown) as number[]);
        },
        [mode, tonic]
    );

    if (error) return <pre>{error.toString()}</pre>;
    if (!native) return <pre>LOADING</pre>;

    return (
        <>
            <h3>Mode</h3>
            <ul>
                {Object.keys(native.modes).map((system, idx) => (
                    <li key={idx}>
                        <h3>{system}</h3>
                        <ul>
                            {native.modes[system].map(({ name, id }) => (
                                <li key={id}>
                                    <Button
                                        variant={
                                            id === mode ? "contained" : "text"
                                        }
                                        color={
                                            id === mode ? "primary" : "default"
                                        }
                                        onClick={() => setMode(id)}
                                    >
                                        {name}
                                    </Button>
                                </li>
                            ))}
                        </ul>
                    </li>
                ))}
            </ul>
            <h3>Tonic</h3>
            <ul>
                {native.tonics.map((name, idx) => (
                    <li key={idx}>
                        <Button
                            variant={idx === tonic ? "contained" : "text"}
                            color={idx === tonic ? "primary" : "default"}
                            onClick={() => setTonic(idx)}
                        >
                            {name}
                        </Button>
                    </li>
                ))}
            </ul>
        </>
    );
}
