package ai.cirkl.jauri.core;

import java.util.Objects;

@SuppressWarnings({"BusyWait", "CallToPrintStackTrace"})
public class Jauri {
    static {
        System.loadLibrary("JWebView");
    }

    private static native void createWebview(String title, int width, int height, String content);
    public static native boolean isWebviewRunning();

    public static void launch(String title, int width, int height) {
        try {
            String content = new String(Objects.requireNonNull(Jauri.class.getResourceAsStream("/build/index.html")).readAllBytes());
            createWebview(title, width, height, content);
        } catch(Exception e) {
            e.printStackTrace();
            createWebview(title, width, height, "Failed to read index.html, did the build fail?");
        }

        while(Jauri.isWebviewRunning()) {
            try {
                Thread.sleep(100);
            } catch(InterruptedException ignored) {
            }
        }
    }
}
