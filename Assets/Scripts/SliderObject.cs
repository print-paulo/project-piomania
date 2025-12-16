using UnityEngine;

public class SliderObject : MonoBehaviour
{
    public float noteTime; // Start time of the hold
    public float endTime; // End time of the hold
    public float scrollSpeed = 10f;

    private float judgementLineY = -4f;
    private float spawnY;

    // Visual components (these should be CHILDREN of this GameObject)
    public Transform sliderHead; // The hit circle at the top
    public Transform sliderBody; // The elongated body
    public Transform sliderTail; // The end circle

    private bool isHolding = false;
    private bool hasStarted = false;
    private float holdDuration;

    void Start()
    {
        spawnY = transform.position.y;
        holdDuration = endTime - noteTime;

        // Calculate speed based on Conductor's travel time
        if (Conductor.instance != null && Conductor.instance.noteTravelTime > 0)
        {
            float totalDistance = spawnY - judgementLineY;
            scrollSpeed = totalDistance / Conductor.instance.noteTravelTime;
        }

        // DEBUG: Check if references are assigned
        Debug.Log($"[SLIDER START] Head={sliderHead != null}, Body={sliderBody != null}, Tail={sliderTail != null}");
        Debug.Log($"[SLIDER START] SpawnY={spawnY}, ScrollSpeed={scrollSpeed}, Duration={holdDuration:F2}s");
        Debug.Log($"[SLIDER START] Position={transform.position}");

        // Make sure all parts are active
        if (sliderHead != null) sliderHead.gameObject.SetActive(true);
        if (sliderBody != null) sliderBody.gameObject.SetActive(true);
        if (sliderTail != null) sliderTail.gameObject.SetActive(true);

        // Setup initial local positions for the slider parts
        SetupSliderParts();
    }

    void SetupSliderParts()
    {
        float visualLength = holdDuration * scrollSpeed;

        // FIXED VALUES - Adjust these to match your sprites
        float headOffset = 0f; // Distance from head center to where body should start
        float tailOffset = 0.288f; // Distance from body end to tail center

        // Position Head at local (0, 0, 0) - this is where we hit (at parent position)
        if (sliderHead != null)
        {
            sliderHead.localPosition = Vector3.zero;
        }

        // Position Body starting after the head offset
        if (sliderBody != null)
        {
            float bodyCenter = headOffset + (visualLength / 2f);
            sliderBody.localPosition = new Vector3(0, bodyCenter, 0);
            sliderBody.localScale = new Vector3(1f, visualLength, 1f);
        }

        // Position Tail after body + tail offset
        if (sliderTail != null)
        {
            float tailPosition = headOffset + visualLength + tailOffset;
            sliderTail.localPosition = new Vector3(0, tailPosition, 0);
        }

        Debug.Log($"[SLIDER SETUP] HeadOffset={headOffset}, TailOffset={tailOffset}, BodyLength={visualLength:F2}, TailFinalPos={headOffset + visualLength + tailOffset:F2}");
    }

    void Update()
    {
        if (Conductor.instance == null) return;

        float songPos = Conductor.instance.songPosition;

        if (float.IsInfinity(songPos) || float.IsNaN(songPos)) return;

        // Handle holding behavior - keep slider head at judgement line
        if (isHolding)
        {
            // Keep head clamped at judgement line
            transform.position = new Vector3(transform.position.x, judgementLineY, transform.position.z);

            // Optionally: shrink the slider as it's being held
            UpdateHoldingVisuals(songPos);
        }
        else
        {
            // Move normally if not holding
            UpdatePosition(songPos);
        }

        // Destroy if completely passed
        if (songPos > endTime + 1f)
        {
            Destroy(gameObject);
        }
    }

    void UpdateHoldingVisuals(float songPos)
    {
        // Calculate how much of the slider has been consumed
        float remaining = endTime - songPos;

        if (remaining <= 0)
        {
            // Slider is complete, will be destroyed by Lane
            return;
        }

        // FIXED VALUES - must match SetupSliderParts
        float headOffset = -0.288f;
        float tailOffset = 0f;

        // Shrink the body and move tail down as slider is held
        float remainingLength = Mathf.Max(remaining * scrollSpeed, 1f);

        if (sliderBody != null)
        {
            float bodyCenter = headOffset + (remainingLength / 2f);
            sliderBody.localPosition = new Vector3(0, bodyCenter, 0);
            sliderBody.localScale = new Vector3(1f, remainingLength, 1f);
        }

        if (sliderTail != null)
        {
            float tailPosition = headOffset + remainingLength + tailOffset;
            sliderTail.localPosition = new Vector3(0, tailPosition, 0);
        }
    }

    void UpdatePosition(float songPos)
    {
        if (isHolding) return; // Don't update position while holding

        // Calculate head position (same logic as normal notes)
        float timeUntilHit = noteTime - songPos;
        float newY = judgementLineY + (timeUntilHit * scrollSpeed);
        newY = Mathf.Min(newY, spawnY);

        // Move the entire slider
        transform.position = new Vector3(transform.position.x, newY, transform.position.z);
    }

    public void StartHold()
    {
        isHolding = true;
        hasStarted = true;
        Debug.Log($"[SLIDER] Started holding at songPos={Conductor.instance.songPosition:F2}");
    }

    public void ReleaseHold()
    {
        isHolding = false;
        Debug.Log($"[SLIDER] Released hold at songPos={Conductor.instance.songPosition:F2}");
    }

    public bool IsHolding()
    {
        return isHolding;
    }

    public bool HasStarted()
    {
        return hasStarted;
    }

    // Helper to get head position in world space (for hit detection)
    public Vector3 GetHeadPosition()
    {
        if (sliderHead != null)
            return sliderHead.position;
        return transform.position;
    }
}