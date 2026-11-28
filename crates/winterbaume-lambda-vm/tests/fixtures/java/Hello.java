import java.util.HashMap;
import java.util.Map;

/** A Lambda handler without dependencies: the runtime maps the event to a Map. */
public class Hello {
    public Map<String, Object> handleRequest(Map<String, Object> event) {
        Map<String, Object> result = new HashMap<>();
        result.put("runtime", "java");
        result.put("version", System.getProperty("java.specification.version"));
        result.put("event", event);
        return result;
    }
}
