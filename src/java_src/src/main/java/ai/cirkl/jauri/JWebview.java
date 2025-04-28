package ai.cirkl.jauri;

import java.util.Objects;

@SuppressWarnings("BusyWait")
public class JWebview {
    static {
        System.loadLibrary("JWebView");
    }

    private static native void createWebview(String title, int width, int height, String content);

    public static void launch(String title, int width, int height) {
        try {
            String content = new String(Objects.requireNonNull(JWebview.class.getResourceAsStream("/build/index.html")).readAllBytes());
            createWebview(title, width, height, content);
        } catch(Exception e) {
            createWebview(title, width, height, "Failed to read index.html, did the build fail?");
        }

        while (JWebview.isWebviewRunning()) {
            try {
                Thread.sleep(100);
            } catch (InterruptedException ignored) {}
        }
    }

    public static native boolean isWebviewRunning();
}
