using UnityEngine;

public class NoteObject : MonoBehaviour
{
    public float noteTime;
    public float scrollSpeed = 5f; // Default safety value

    // Set this equal to your Lane's judgement line (e.g. -3 or -4)
    private float judgementLineY = -4f;
    private float spawnY; // Store the initial Y position

    void Start()
    {
        // Store the spawn Y position
        spawnY = transform.position.y;

        // Calculate speed based on Conductor's travel time
        if (Conductor.instance != null && Conductor.instance.noteTravelTime > 0)
        {
            float totalDistance = spawnY - judgementLineY;
            scrollSpeed = totalDistance / Conductor.instance.noteTravelTime;
        }
    }

    void Update()
    {
        // Safety check: If no conductor exists, do nothing
        if (Conductor.instance == null) return;

        float songPos = Conductor.instance.songPosition;

        // Safety check: Avoid calculating position if time is invalid (very negative at start)
        // This prevents Infinity errors in the first few frames
        if (float.IsInfinity(songPos) || float.IsNaN(songPos)) return;

        // Movement logic: Calculate time remaining until note hits the line
        float timeUntilHit = noteTime - songPos;

        // Y position is the judgement line + (remaining time * speed)
        float newY = judgementLineY + (timeUntilHit * scrollSpeed);

        // Important: Limit position to not exceed spawn point
        newY = Mathf.Min(newY, spawnY);

        // Apply position only on Y axis, keeping original X and Z from lane
        transform.position = new Vector3(transform.position.x, newY, transform.position.z);

        // Destroy if note passed too far (Miss)
        if (songPos > noteTime + 1f)
        {
            Destroy(gameObject);
        }
    }
}