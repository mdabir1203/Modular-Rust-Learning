from transformers import pipeline
from transformers import AutoModel

access_token="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"

model = AutoModel.from_pretrained("meta-llama/Meta-Llama-3-8B", token=access_token)



def generate_text(prompt):
    # Load Llama 3 model from Hugging Face
    llama3_model = pipeline("text-generation", model="meta-llama/Meta-Llama-3-8B")

    # Generate text using the Llama 3 model
    generated_text = llama3_model(prompt, max_length=50, do_sample=True)

    # Return the generated text
    return generated_text[0]['generated_text']

# Example usage
if __name__ == "__main__":
    prompt = "Once upon a time"
    text = generate_text(prompt)
    print(f"Generated text: {text}")
