## One project on every desktop {#platforms}

The desktop interface is the program a musician runs on a desktop computer to arrange, edit, mix,
record and export a project. euphony supports three desktop operating systems, and the desktop
interface runs on each of them. Each copy of the desktop interface is a client of the session host
through the client protocol, as every other interface is: it shows the project and sends commands,
and the session host accepts or refuses each command. The main views are the arrangement, the
mixer, the piano roll and the automation view. The services of an operating system are its menus,
its file dialogs, its clipboard and drag and drop, its text input and input methods, its window
management and its accessibility services.

### EU-0018-001 {#EU-0018-001}

The desktop interface MUST run on each desktop operating system euphony supports.

### EU-0018-002 {#EU-0018-002}

A project MUST open on each supported desktop operating system with the same timing, routing, automation and sound definitions.

### EU-0018-003 {#EU-0018-003}

The same revision of a project MUST render the same audio on each supported desktop operating system.

### EU-0018-004 {#EU-0018-004}

The desktop interface MUST use the operating system's own menus, file dialogs, clipboard, drag and drop, window management and accessibility services.

### EU-0018-005 {#EU-0018-005}

Every main view, navigation path and editing command MUST pass the same conformance cases on each supported desktop operating system.

### EU-0018-006 {#EU-0018-006}

The desktop interface MUST reach the session host only through the client protocol every interface uses.

### EU-0018-007 {#EU-0018-007}

The desktop interface MUST NOT hold the authority to accept a change to a project.
