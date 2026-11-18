## Station pairing and stream confidentiality {#pairing}

On its first start a station makes its device key pair and keeps the private key on the station. It
advertises itself on the local network, and that advertisement is only a hint. Pairing binds the
station's key to one sink and that sink's gain limit: the owner's desktop or phone shows the
station's key fingerprint and a short confirmation number, the station shows the same number on its
lights, a person presses the station's encoder switch to show they are present, and the owner
approves the pairing on the desktop or phone. Each computer of the session keeps the list of paired
stations, and checks it on every connection and every command. A station that isn't on the host's
network joins through the session's authenticated invitation path. By default the audio of a station
is an authenticated, encrypted stream that only the paired station can decode. On a network the
owner marks as theirs, the host can send the stream as an unencrypted standard network audio stream,
so that the station or another standard receiver can play it; there, pairing still decides which
station the host sends to and whose commands it takes, but any receiver on that network can hear a
stream in use. Control traffic is authenticated and encrypted on every network. Unpairing can't take
back audio already played.

### EU-0015-057 {#EU-0015-057}

A station MUST make its device key pair on its first start.

### EU-0015-058 {#EU-0015-058}

The host MUST treat a station's network advertisement as an untrusted hint.

### EU-0015-059 {#EU-0015-059}

The host MUST NOT address audio or meters to a station that isn't paired.

### EU-0015-060 {#EU-0015-060}

The host MUST refuse every command of a station that isn't paired.

### EU-0015-061 {#EU-0015-061}

During pairing, the owner's desktop or phone MUST show the station's key fingerprint and a short confirmation number.

### EU-0015-062 {#EU-0015-062}

During pairing, the station MUST show the same confirmation number on its lights.

### EU-0015-063 {#EU-0015-063}

Pairing MUST require a press of the station's switch.

### EU-0015-064 {#EU-0015-064}

Pairing MUST require the owner's approval on the desktop or phone.

### EU-0015-065 {#EU-0015-065}

A pairing whose confirmation numbers don't match MUST fail.

### EU-0015-066 {#EU-0015-066}

Pairing MUST bind the station to one sink and that sink's gain limit.

### EU-0015-067 {#EU-0015-067}

The list of paired stations MUST be checked on every connection and every command.

### EU-0015-068 {#EU-0015-068}

A station that isn't on the host's network MUST join through the session's authenticated invitation path.

### EU-0015-069 {#EU-0015-069}

The host MUST refuse a second station that presents a paired station's identity.

### EU-0015-070 {#EU-0015-070}

Unpairing a station MUST end its stream and its control session at once.

### EU-0015-071 {#EU-0015-071}

By default, a station's audio stream MUST be encrypted so that only the paired station can decode it.

### EU-0015-072 {#EU-0015-072}

On a network the owner marks as theirs, the host MAY send a station's audio as an unencrypted standard network audio stream.

### EU-0015-073 {#EU-0015-073}

The host MUST NOT send a station's audio unencrypted on a network the owner hasn't marked as theirs.

### EU-0015-074 {#EU-0015-074}

The host MUST stop an unencrypted station stream when the network's identity no longer matches the owner's mark.

### EU-0015-075 {#EU-0015-075}

Control traffic between a station and the host MUST be authenticated and encrypted on every network.

### EU-0015-113 {#EU-0015-113}

A station MUST keep the private key of its device key pair on the station.
