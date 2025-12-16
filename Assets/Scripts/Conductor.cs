using UnityEngine;
using System.Collections.Generic;
using System.IO;

public class Conductor : MonoBehaviour
{
    public static Conductor instance;

    [Header("Configurations")]
    public AudioSource songSource;
    public string fileMapName; // Name of the map file (with extension)
    public float noteTravelTime = 2.0f; // Time it takes for a note to travel from spawn to hit position
    public float audioOffset = 0f;
    public float initialDelay = 3.0f; // Delay before the song starts playing

    [Header("References")]
    public Lane[] lanes;

    // Time variables (for debug reading)
    public float songPosition;
    public float dspSongTime;

    // Song data    
    public SongData songData;
    public int nextNoteIndex = 0;
    private bool musicStarted = false;

    void Awake()
    {
        instance = this;
    }

    void Start()
    {
        string path = Path.Combine(Application.streamingAssetsPath, fileMapName);

        songData = new SongData();
        songData.notes = OsuParser.ParseOsu(path);

        if (songData.notes != null && songData.notes.Count > 0) // If notes were loaded successfully
        {
            songData.notes.Sort((n1, n2) => n1.time.CompareTo(n2.time)); // Ensure notes are sorted by time
        }

        dspSongTime = (float)AudioSettings.dspTime;

        // Schedule the song to play after (initialDelay + noteTravelTime)
        double startTime = AudioSettings.dspTime + initialDelay + noteTravelTime;
        songSource.PlayScheduled(startTime);
        musicStarted = true;
    }

    void Update()
    {
        if (!musicStarted) return;

        songPosition = (float)(AudioSettings.dspTime - dspSongTime) - initialDelay - noteTravelTime;

        if (songData != null && songData.notes != null)
        {
            while (nextNoteIndex < songData.notes.Count)
            {
                NoteInfo nextNote = songData.notes[nextNoteIndex];

                if (songPosition >= nextNote.time - noteTravelTime)
                {
                    if (nextNote.laneIndex < lanes.Length && nextNote.laneIndex >= 0)
                    {
                        // Pass endTime to distinguish sliders from normal notes
                        lanes[nextNote.laneIndex].SpawnNote(nextNote.time, nextNote.endTime);
                    }
                    nextNoteIndex++;
                }
                else
                {
                    break;
                }
            }
        }
    }
}