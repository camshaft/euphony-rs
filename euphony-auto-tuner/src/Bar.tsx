import { makeStyles, createStyles, Theme } from "@material-ui/core/styles";
import AppBar from "@material-ui/core/AppBar";
import Toolbar from "@material-ui/core/Toolbar";
import Typography from "@material-ui/core/Typography";
import Button from "@material-ui/core/Button";
import IconButton from "@material-ui/core/IconButton";
import InputLabel from "@material-ui/core/InputLabel";
import MenuItem from "@material-ui/core/MenuItem";
import FormHelperText from "@material-ui/core/FormHelperText";
import FormControl from "@material-ui/core/FormControl";
import Select from "@material-ui/core/Select";
import { useMIDI } from "@react-midi/hooks";

const useStyles = makeStyles((theme) => ({
    root: {
        flexGrow: 1,
    },
    menuButton: {
        marginRight: theme.spacing(2),
    },
    title: {
        flexGrow: 1,
    },
}));

export function Bar({
    setInput,
    input,
    setOutput,
    output,
}: {
    setInput: (v: string) => void;
    input: string;
    setOutput: (v: string) => void;
    output: string;
}) {
    const classes = useStyles();
    const { inputs, outputs } = useMIDI();

    return (
        <div className={classes.root}>
            <AppBar position="static">
                <Toolbar>
                    <IconButton
                        edge="start"
                        className={classes.menuButton}
                        color="inherit"
                        aria-label="menu"
                    >
                        Menu
                    </IconButton>
                    <div className={classes.title} />
                    <Picker
                        set={setInput}
                        value={input}
                        values={inputs}
                        label="Input"
                    />
                    <Picker
                        set={setOutput}
                        value={output}
                        values={outputs}
                        label="Output"
                    />
                </Toolbar>
            </AppBar>
        </div>
    );
}

const usePickerStyles = makeStyles((theme: Theme) =>
    createStyles({
        formControl: {
            margin: theme.spacing(1),
            minWidth: 120,
        },
        selectEmpty: {
            marginTop: theme.spacing(2),
        },
        select: {
            color: theme.palette.primary.contrastText,
        },
    })
);

interface Thing {
    id: string;
    name: string;
}

function Picker({
    set,
    value,
    values,
    label,
}: {
    set: (v: string) => void;
    value: string;
    label: string;
    values: Thing[];
}) {
    const classes = usePickerStyles();

    const id = `picker-midi-${label}`;

    const handleChange = (event: React.ChangeEvent<{ value: unknown }>) => {
        set(event.target.value as string);
    };

    return (
        <FormControl className={classes.formControl}>
            <InputLabel className={classes.select} id={`${id}-label`}>
                {label}
            </InputLabel>
            <Select
                labelId={`${id}-label`}
                id={id}
                value={value}
                onChange={handleChange}
                className={classes.select}
            >
                {values.map((input, key) => (
                    <MenuItem key={key} value={input.id}>
                        {input.name}
                    </MenuItem>
                ))}
            </Select>
        </FormControl>
    );
}
