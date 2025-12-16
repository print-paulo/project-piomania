using UnityEngine;
using UnityEngine.UI;

public class GameManager : MonoBehaviour
{
    public static GameManager instance;

    public int score = 0;
    public int multiplier = 1;

    public Text scoreText;

    private void Start()
    {
        instance = this;
    }

    public void AddScore(int scored)
    {
        score += scored * multiplier;

        if (scoreText != null)
            scoreText.text = "Score: " + score;
   
        Debug.Log("Score: " + score);
    }
}