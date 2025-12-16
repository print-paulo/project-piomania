using UnityEngine;
using System.Collections.Generic;

public class Lane : MonoBehaviour
{
    public KeyCode inputKey;
    public List<GameObject> notesOnLane = new List<GameObject>();
    public Transform noteSpawnPoint;
    public GameObject notePrefab; // Normal note prefab
    public GameObject sliderPrefab; // Slider prefab
    public float judgementLineY = -4f;
    public float errorMargin = 0.5f;
    public float missThresholdY = -6f;

    private bool isKeyHeld = false;

    private void Update()
    {
        // Clean up destroyed notes
        while (notesOnLane.Count > 0 && notesOnLane[0] == null)
        {
            notesOnLane.RemoveAt(0);
        }

        // Check for missed notes (passed too far down)
        if (notesOnLane.Count > 0 && notesOnLane[0] != null)
        {
            GameObject firstNote = notesOnLane[0];

            // For sliders, check the head position
            SliderObject slider = firstNote.GetComponent<SliderObject>();
            float checkY = slider != null ? slider.GetHeadPosition().y : firstNote.transform.position.y;

            if (checkY < missThresholdY)
            {
                // Check if it's a slider that wasn't hit
                if (slider == null || !slider.HasStarted())
                {
                    Debug.Log("Miss (too late)!");
                }

                notesOnLane.RemoveAt(0);
                Destroy(firstNote);
            }
        }

        // Handle key press
        if (Input.GetKeyDown(inputKey))
        {
            isKeyHeld = true;
            CheckForHit();
        }

        // Handle key hold for sliders
        if (Input.GetKey(inputKey) && isKeyHeld)
        {
            UpdateSliderHold();
        }

        // Handle key release
        if (Input.GetKeyUp(inputKey))
        {
            isKeyHeld = false;
            ReleaseSlider();
        }
    }

    void CheckForHit()
    {
        if (notesOnLane.Count == 0) return;
        if (notesOnLane[0] == null)
        {
            notesOnLane.RemoveAt(0);
            return;
        }

        GameObject targetNote = notesOnLane[0];

        // Check if it's a slider
        SliderObject slider = targetNote.GetComponent<SliderObject>();

        if (slider != null)
        {
            // For sliders, check if head is at judgement line
            float distance = Mathf.Abs(slider.GetHeadPosition().y - judgementLineY);

            if (distance < errorMargin)
            {
                Debug.Log("Slider Hit! Hold the key!");
                slider.StartHold();

                if (GameManager.instance != null)
                {
                    GameManager.instance.AddScore(50); // Partial score for starting
                }
            }
            else
            {
                Debug.Log("Miss (out of sync)!");
            }
        }
        else
        {
            // Normal note logic
            float distance = Mathf.Abs(targetNote.transform.position.y - judgementLineY);

            if (distance < errorMargin)
            {
                Debug.Log("Hit! +100 Points");

                if (GameManager.instance != null)
                {
                    GameManager.instance.AddScore(100);
                }

                notesOnLane.RemoveAt(0);
                Destroy(targetNote);
            }
            else
            {
                Debug.Log("Miss (out of sync)!");
            }
        }
    }

    void UpdateSliderHold()
    {
        if (notesOnLane.Count == 0) return;
        if (notesOnLane[0] == null) return;

        SliderObject slider = notesOnLane[0].GetComponent<SliderObject>();

        if (slider != null && slider.HasStarted())
        {
            // Check if slider is complete
            float songPos = Conductor.instance.songPosition;

            if (songPos >= slider.endTime - 0.05f) // Small tolerance
            {
                Debug.Log("Slider Complete! +100 Points");

                if (GameManager.instance != null)
                {
                    GameManager.instance.AddScore(100);
                }

                GameObject completedSlider = notesOnLane[0];
                notesOnLane.RemoveAt(0);
                Destroy(completedSlider);
            }
        }
    }

    void ReleaseSlider()
    {
        if (notesOnLane.Count == 0) return;
        if (notesOnLane[0] == null) return;

        SliderObject slider = notesOnLane[0].GetComponent<SliderObject>();

        if (slider != null && slider.IsHolding())
        {
            float songPos = Conductor.instance.songPosition;

            // Check if released too early
            if (songPos < slider.endTime - 0.1f) // 0.1s tolerance
            {
                Debug.Log("Slider Break! Released too early!");

                notesOnLane.RemoveAt(0);
                Destroy(slider.gameObject);
            }
            else
            {
                slider.ReleaseHold();
            }
        }
    }

    public void SpawnNote(float time, float endTime = 0)
    {
        GameObject newNote;

        // Check if it's a hold note (slider)
        if (endTime > time)
        {
            Debug.Log($"[SLIDER] Spawning slider: time={time:F2}s, endTime={endTime:F2}s, duration={endTime - time:F2}s");

            if (sliderPrefab == null)
            {
                Debug.LogError("[SLIDER] SliderPrefab is NULL! Assign it in the Inspector!");
                return;
            }

            newNote = Instantiate(sliderPrefab, noteSpawnPoint.position, Quaternion.identity);

            SliderObject sliderScript = newNote.GetComponent<SliderObject>();
            if (sliderScript == null)
            {
                Debug.LogError("[SLIDER] SliderObject script not found on prefab!");
                return;
            }

            sliderScript.noteTime = time;
            sliderScript.endTime = endTime;

            Debug.Log($"[SLIDER] Slider spawned successfully at position {noteSpawnPoint.position}");
        }
        else
        {
            Debug.Log($"[NOTE] Spawning normal note: time={time:F2}s");
            newNote = Instantiate(notePrefab, noteSpawnPoint.position, Quaternion.identity);

            NoteObject noteScript = newNote.GetComponent<NoteObject>();
            noteScript.noteTime = time;
        }

        notesOnLane.Add(newNote);
    }
}