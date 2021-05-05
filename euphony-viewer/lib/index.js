import WaveformPlaylist from 'waveform-viewer'

window.EuphonyViewer = async container => {
  // TODO configure

  const playlist = WaveformPlaylist.init({
    samplesPerPixel: 1000,
    waveHeight: 100,
    container,
    timescale: true,
    state: 'cursor',
    colors: {
      waveOutlineColor: '#E0EFF1'
    },
    controls: {
      show: true, //whether or not to include the track controls
      width: 200 //width of controls in pixels
    },
    zoomLevels: [500, 1000, 3000, 5000]
  })

  const project = await fetch('project.json')
  const tracks = Object.keys(project.tracks).map(name => ({
    src: project.tracks[name],
    name
  }))

  await playlist.load(tracks)

  playlist
}
