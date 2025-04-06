import os
import google.generativeai as genai


genai.configure(api_key="AIzaSyBJbKTNWs2JcLErkpvXw7cOpKNbzUOsV1w")

try:
    model = genai.GenerativeModel('gemini-1.5-pro-latest') # 你可以选择其他合适的模型
except Exception as e:
    print(f"初始化模型时出错: {e}")
    exit()

# --- 3. 准备你的提示 (Prompt) ---
prompt_text = input("请输入你的提示: ")

# --- 4. 调用 API 生成内容 ---
try:
    response = model.generate_content(prompt_text)

    # --- 5. 打印结果 ---
    # 检查是否有有效内容生成
    if response.candidates and response.candidates[0].content.parts:
        print("--- Gemini's Response ---")
        print(response.text)
    else:
        print("--- 未能生成内容 ---")
        print(f"提示反馈: {response.prompt_feedback}")
        if response.candidates:
            print(f"完成原因: {response.candidates[0].finish_reason}")

except Exception as e:
    print(f"调用 Gemini API 时发生错误: {e}")
