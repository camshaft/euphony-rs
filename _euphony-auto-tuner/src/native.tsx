import { useState, useEffect } from "react";

export interface Native {
  process(
    input: Uint8Array,
    output: Uint8Array,
    mode: number,
    tonic: number
  ): number;
  modes: Modes;
  tonics: string[];
}

export type Modes = { [name: string]: Mode[] };

export interface Mode {
  id: number;
  name: string;
}

export function useNative() {
  const [value, setNative] = useState<Native | null>(null);
  const [error, setError] = useState<Error | null>(null);

  async function resolve() {
    try {
      const mod = await import("auto-tuner-rs");
      const modes = {} as Modes;
      for (let i = 0; i < mod.mode_len(); i++) {
        const system_id = mod.mode_system(i);
        const system = modes[system_id] || (modes[system_id] = []);
        const mode = {} as Mode;
        mode.name = mod.mode_name(i);
        mode.id = i;
        system.push(mode);
      }
      const tonics = [];
      for (let i = 0; i < mod.tonic_len(); i++) {
        const name = mod.tonic_name(i);
        tonics.push(name);
      }
      setNative({ process: mod.process, modes, tonics });
    } catch (e) {
      setError(new Error(e));
    }
  }

  useEffect(() => {
    resolve();
  }, []);

  return [value, error] as [Native | null, Error | null];
}
