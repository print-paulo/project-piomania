using UnityEngine;
using System.Collections.Generic;

public class Lane : MonoBehaviour
{
    public KeyCode inputKey;
    public List<GameObject> notesOnLane = new List<GameObject>();
    public Transform noteSpawnPoint;
    public GameObject notePrefab; // Normal note prefab
    public GameObject sliderPrefab; // Slider prefab
    public float judgementLineY;
    private const float errorMarginMiss = 1f;
    private const float errorMarginGood = 0.75f;
    private const float errorMarginGreat = 0.5f;
    private const float errorMarginPerfect = 0.25f;
    private const float errorMarginLimiter = 3f;

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

            // Don't check miss threshold for sliders that are being held
            if (slider != null && slider.IsHolding())
            {
                // Skip miss check for active sliders
            }
            else
            {
                float checkY = slider != null ? slider.GetHeadPosition().y : firstNote.transform.position.y;

                if (checkY < missThresholdY)
                {
                    // Check if it's a slider that wasn't hit
                    if (slider == null || !slider.HasStarted())
                    {
                        // Miss detected
                        Debug.Log("Missed note.");
                        if (GameManager.instance != null)
                        {

                            GameManager.instance.ResetCombo();
                            GameManager.instance.AddScore(0);
                            GameManager.instance.RecordHit("Miss");
                        }
                    }

                    notesOnLane.RemoveAt(0);
                    Destroy(firstNote);
                }
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

            if (distance <= errorMarginMiss)
            {
                slider.StartHold();

                if (distance <= errorMarginPerfect)
                {
                    // Perfect
                    if (GameManager.instance != null)
                    {
                        GameManager.instance.AddCombo();
                        GameManager.instance.AddScore(300); // Partial score for starting
                        GameManager.instance.RecordHit("Perfect");
                    }
                }
                else if (distance <= errorMarginGreat)
                {
                    // Great
                    if (GameManager.instance != null)
                    {
                        GameManager.instance.AddCombo();
                        GameManager.instance.AddScore(200);
                        GameManager.instance.RecordHit("Great");
                    }
                }
                else if (distance <= errorMarginGood)
                {
                    // Good
                    if (GameManager.instance != null)
                    {
                        GameManager.instance.AddCombo();
                        GameManager.instance.AddScore(100);
                        GameManager.instance.RecordHit("Good");
                    }
                }
            }
            else if (distance >= errorMarginLimiter)
            {
                // Too far for hit, but within limiter - do nothing
            }
            else
            {
                // Miss
                Debug.Log("Missed slider note.");
                if (GameManager.instance != null)
                {
                    
                    GameManager.instance.ResetCombo();
                    GameManager.instance.AddScore(0);
                    GameManager.instance.RecordHit("Miss");
                }
            }
        }
        else
        {
            // Normal note logic
            float distance = Mathf.Abs(targetNote.transform.position.y - judgementLineY);

            if (distance <= errorMarginMiss)
            {
                if (distance <= errorMarginPerfect)
                {
                    // Perfect
                    if (GameManager.instance != null)
                    {
                        GameManager.instance.AddCombo();
                        GameManager.instance.AddScore(300); // Partial score for starting
                        GameManager.instance.RecordHit("Perfect");
                    }
                }
                else if (distance <= errorMarginGreat)
                {
                    // Great
                    if (GameManager.instance != null)
                    {
                        GameManager.instance.AddCombo();
                        GameManager.instance.AddScore(200);
                        GameManager.instance.RecordHit("Great");
                    }
                }
                else if (distance <= errorMarginGood)
                {
                    // Good
                    if (GameManager.instance != null)
                    {
                        GameManager.instance.AddCombo();
                        GameManager.instance.AddScore(100);
                        GameManager.instance.RecordHit("Good");
                    }
                }

                notesOnLane.RemoveAt(0);
                Destroy(targetNote);
            }
            else if (distance >= errorMarginLimiter)
            {
                // Too far for hit, but within limiter - do nothing
            }
            else
            {
                // Miss
                Debug.Log("Missed slider note.");
                if (GameManager.instance != null)
                {
                    GameManager.instance.ResetCombo();
                    GameManager.instance.AddScore(0);
                    GameManager.instance.RecordHit("Miss");
                }
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
            if (sliderPrefab == null)
            {
                return;
            }

            newNote = Instantiate(sliderPrefab, noteSpawnPoint.position, Quaternion.identity);

            SliderObject sliderScript = newNote.GetComponent<SliderObject>();
            if (sliderScript == null)
            {
                return;
            }

            sliderScript.noteTime = time;
            sliderScript.endTime = endTime;
        }
        else
        {
            newNote = Instantiate(notePrefab, noteSpawnPoint.position, Quaternion.identity);

            NoteObject noteScript = newNote.GetComponent<NoteObject>();
            noteScript.noteTime = time;
        }

        notesOnLane.Add(newNote);
    }
}