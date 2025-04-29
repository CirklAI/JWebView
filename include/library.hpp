#ifndef JWEBVIEW_LIBRARY_HPP
#define JWEBVIEW_LIBRARY_HPP

#include <jni.h>

#ifdef __cplusplus
extern "C" {
#endif

JNIEXPORT void JNICALL Java_ai_cirkl_jauri_core_Jauri_createWebview(
        JNIEnv* env, jclass clazz, jstring title, jint width, jint height, jstring content);

    JNIEXPORT jboolean JNICALL Java_ai_cirkl_jauri_core_Jauri_isWebviewRunning(
        JNIEnv* env, jobject obj);

#ifdef __cplusplus
}
#endif

#endif //JWEBVIEW_LIBRARY_HPP
