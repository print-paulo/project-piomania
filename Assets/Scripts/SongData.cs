using System.Collections.Generic;

[System.Serializable]

public class NoteInfo
{
    public float time; // Start of the note
    public int laneIndex; // Which lane is the note
    public float endTime; // End of the note (for hold notes)
}

[System.Serializable]

public class SongData
{
    public string songName;
    public List<NoteInfo> notes; // List of notes in the song
}