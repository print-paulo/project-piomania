using UnityEngine;
using UnityEngine.UI;

public class GameManager : MonoBehaviour
{
    public static GameManager instance;

    public int scrollSpeed = 10;
    public int score = 0;
    public int combo = 0;
    public int maxCombo = 0;
    public int multiplier = 1; // Based on combo
    public int perfect = 0;
    public int great = 0;
    public int good = 0;
    public int miss = 0;

    public Text scoreText;

    private void Start()
    {
        instance = this;
    }

    public void UpdateMultiplier(int newMultiplier)
    {
        multiplier = newMultiplier;
        Debug.Log("Multiplier updated to: " + multiplier);
    }

    public void ResetCombo()
    {
        combo = 0;
        multiplier = 1;
        Debug.Log("Combo reset.");
    }

    public void AddCombo()
    {
        combo++;
        if (combo > maxCombo)
            maxCombo = combo;
        // Update multiplier based on combo thresholds
        Debug.Log("Combo: " + combo);
    }

    public void RecordHit(string hitType)
    {
        switch (hitType)
        {
            case "Perfect":
                perfect++;
                break;
            case "Great":
                great++;
                break;
            case "Good":
                good++;
                break;
            case "Miss":
                miss++;
                break;
        }
        Debug.Log("Hits - Perfect: " + perfect + ", Great: " + great + ", Good: " + good + ", Miss: " + miss);
    }

    public void ResetStats()
    {
        score = 0;
        combo = 0;
        maxCombo = 0;
        multiplier = 1;
        perfect = 0;
        great = 0;
        good = 0;
        miss = 0;
        if (scoreText != null)
            scoreText.text = "Score: " + score;
        Debug.Log("Game stats reset.");
    }

    public void AddScore(int scored)
    {
        if (combo >= 300)
            multiplier = 4;
        else if (combo >= 200)
            multiplier = 3;
        else if (combo >= 100)
            multiplier = 2;
        else
            multiplier = 1;
        
        score += scored * multiplier;
        if (scoreText != null)
            scoreText.text = "Score: " + score;
        Debug.Log("Score added: " + (scored * multiplier) + " | Total Score: " + score + " | Multiplier: " + multiplier);
    }
}