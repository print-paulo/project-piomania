using UnityEngine;

public class SliderObject : MonoBehaviour
{
    [Header("Note Data")]
    public float noteTime;
    public float endTime;
    public float scrollSpeed;

    [Header("Visual Adjustments (Positive values)")]
    // If body needs to start slightly above head center, increase this
    public float bodyStartOffset = 0f;

    // Use this to stretch or shrink the body to fit the tail
    public float bodyHeightCorrection = 0f;

    private float judgementLineY = -4f; // Make sure this matches your Lane
    private float spawnY;
    private float holdDuration;

    public Transform sliderHead;
    public Transform sliderBody;
    public Transform sliderTail;

    // Internal states
    private bool isHolding = false;
    private bool hasStarted = false;

    void Start()
    {
        spawnY = transform.position.y;
        holdDuration = endTime - noteTime;

        // Calculate speed if Conductor is present
        if (Conductor.instance != null && Conductor.instance.noteTravelTime > 0)
        {
            float totalDistance = spawnY - judgementLineY;
            scrollSpeed = totalDistance / Conductor.instance.noteTravelTime;
        }

        // Ensure Body uses Bottom Pivot
        if (sliderBody != null)
        {
            sliderBody.localRotation = Quaternion.identity;
        }

        UpdateVisuals(holdDuration * scrollSpeed); // Initial setup
    }

    void Update()
    {
        if (Conductor.instance == null) return;

        float songPos = Conductor.instance.songPosition;

        // If holding, lock head to judgement line
        if (isHolding)
        {
            transform.position = new Vector3(transform.position.x, judgementLineY, transform.position.z);

            // Calculate remaining note length
            float remainingTime = endTime - songPos;
            UpdateVisuals(Mathf.Max(0, remainingTime * scrollSpeed));
        }
        else
        {
            // Normal downward movement
            float timeUntilHit = noteTime - songPos;
            float newY = judgementLineY + (timeUntilHit * scrollSpeed);

            // Lock to spawn position to prevent going to infinity
            newY = Mathf.Min(newY, spawnY);

            transform.position = new Vector3(transform.position.x, newY, transform.position.z);
        }

        // Destroy if finished (passed end time + margin)
        if (songPos > endTime + 0.5f)
        {
            Destroy(gameObject);
        }
    }

    void UpdateVisuals(float currentLength)
    {
        // If note is finished, hide everything
        if (currentLength <= 0.001f)
        {
            if (sliderBody != null) sliderBody.gameObject.SetActive(false);
            if (sliderTail != null) sliderTail.gameObject.SetActive(false);
            return;
        }

        if (sliderHead != null)
        {
            sliderHead.localPosition = Vector3.zero;
        }

        // It goes to the exact point where the note ends
        if (sliderTail != null)
        {
            sliderTail.gameObject.SetActive(true);
            sliderTail.localPosition = new Vector3(0, currentLength, 0);
        }

        // Body grows from Head (0) to Tail center (currentLength)
        if (sliderBody != null)
        {
            sliderBody.gameObject.SetActive(true);

            // Starts at Head center
            sliderBody.localPosition = Vector3.zero;

            // Stretches to reach Tail center
            sliderBody.localScale = new Vector3(1f, currentLength - 0.45f, 1f);
        }
    }

    // Public methods called by Lane
    public void StartHold()
    {
        isHolding = true;
        hasStarted = true; // Mark that player hit the start
    }

    public void ReleaseHold()
    {
        isHolding = false;
    }

    // Lane uses this to check if input is still held
    public bool IsHolding()
    {
        return isHolding;
    }

    // Lane uses this to not destroy slider if it's already being held
    public bool HasStarted()
    {
        return hasStarted;
    }

    // Lane uses this to calculate hit distance (if Head object exists)
    public Vector3 GetHeadPosition()
    {
        if (sliderHead != null) return sliderHead.position;
        return transform.position;
    }
}