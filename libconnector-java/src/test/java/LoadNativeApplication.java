import de.itdesigners.lukas.connector.jni.NativeConnector;
import org.junit.jupiter.api.Test;

public class LoadNativeApplication {

    @Test
    public void load() {
        // fine if there is no UnsatisfiedLinkError
        var connector = new NativeConnector();
        var info = connector.getInfo();

        System.out.println(info.getVersion());
        System.out.println(info.getProtocolVersion());
    }
}
