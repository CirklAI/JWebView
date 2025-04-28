#include "library.hpp"

#include <iostream>
#include <jni.h>
#include "webview/webview.h"

static bool webviewRunning = false;

JNIEXPORT void JNICALL Java_Jauri_Webview_createWebview(
    JNIEnv* env, jclass clazz, jstring title, jint width, jint height, jstring content) {

    const char* titleChars = env->GetStringUTFChars(title, nullptr);
    const char* contentChars = env->GetStringUTFChars(content, nullptr);
    std::string titleStr(titleChars);
    std::string contentStr(contentChars);
    env->ReleaseStringUTFChars(title, titleChars);
    env->ReleaseStringUTFChars(content, contentChars);

    webviewRunning = true;

    try {
        webview::webview w(false, nullptr);
        w.set_title(titleStr);
        w.set_size(width, height, WEBVIEW_HINT_NONE);
        w.set_html(contentStr);
        w.run();
    } catch (const webview::exception &e) {
        std::cerr << e.what() << '\n';
    }
    webviewRunning = false;
}

JNIEXPORT jboolean JNICALL Java_Jauri_Webview_isWebviewRunning(
    JNIEnv* env, jobject obj) {
    return webviewRunning;
}