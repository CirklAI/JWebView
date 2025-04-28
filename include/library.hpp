#ifndef JWEBVIEW_LIBRARY_HPP
#define JWEBVIEW_LIBRARY_HPP

#include <jni.h>

#ifdef __cplusplus
extern "C" {
#endif

    JNIEXPORT void JNICALL Java_Jauri_Webview_createWebview(
        JNIEnv* env, jclass clazz, jstring title, jint width, jint height, jstring content);

    JNIEXPORT jboolean JNICALL Java_Jauri_Webview_isWebviewRunning(
        JNIEnv* env, jobject obj);

#ifdef __cplusplus
}
#endif

#endif //JWEBVIEW_LIBRARY_HPP
