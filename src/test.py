import speech_recognition as sr
import sys
import os

r = sr.Recognizer()

with open(os.curdir + "/ShellHacks/shellhacks-2018/src/cred.json") as f:
    GOOGLE_CLOUD_SPEECH_CREDENTIALS = f.read()

with sr.Microphone() as source:
    audio = r.listen(source)
    try:
        text = r.recognize_google_cloud(audio,GOOGLE_CLOUD_SPEECH_CREDENTIALS)
        print("{}".format(text))
    except:
        sys.exit(-1)