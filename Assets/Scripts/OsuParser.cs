using UnityEngine;
using System.Collections.Generic;
using System.IO;
using System.Globalization; // For parsing float with invariant culture

public static class OsuParser
{
    public static List<NoteInfo> ParseOsu(string filePath)
    {
        List<NoteInfo> noteList = new List<NoteInfo>();

        if (!File.Exists(filePath))
        {
            Debug.LogError("File not found: " + filePath);
            return null;
        }

        string[] lines = File.ReadAllLines(filePath);

        // 4 Lanes Mapping
        int keyCount = 4;

        bool readingHitObjects = false;

        foreach (string line in lines)
        {
            if (string.IsNullOrWhiteSpace(line)) continue;

            if (line.StartsWith("CircleSize:")) // Circle size = number of lanes
            {
                keyCount = int.Parse(line.Split(':')[1]);
            }

            // Start reading hit objects
            if (line.StartsWith("[HitObjects]"))
            {
                readingHitObjects = true;
                continue;
            }

            if (readingHitObjects)
            {
                string[] parts = line.Split(',');

                if (parts.Length < 4) continue;

                int x = int.Parse(parts[0]);
                float timeMs = float.Parse(parts[2], CultureInfo.InvariantCulture);
                int type = int.Parse(parts[3]);

                int laneIndex = Mathf.FloorToInt(x * keyCount / 512f);
                laneIndex = Mathf.Clamp(laneIndex, 0, keyCount - 1);

                NoteInfo newNote = new NoteInfo();
                newNote.time = timeMs / 1000f; // Convert to seconds
                newNote.laneIndex = laneIndex;

                // Check if it's a hold note (type 128)
                if ((type & 128) != 0)
                {
                    string[] extras = parts[5].Split(':');
                    float endTimeMs = float.Parse(extras[0], CultureInfo.InvariantCulture);
                    newNote.endTime = endTimeMs / 1000f; // Convert to seconds
                }
                else
                {
                    newNote.endTime = 0; // For normal notes, endTime = time
                }

                noteList.Add(newNote);
            }
        }
        return noteList;
    }
}