package net.insprill.customcarmanager.ui.factory;

import javafx.fxml.FXMLLoader;
import javafx.scene.Parent;

import java.io.IOException;

public class FXMLFactory {

    /**
     * Loads an FXML file.
     *
     * @param path The path of the FXML file (internal).
     * @param <T>  The type of the controller.
     * @return A {@link FXMLElement} containing the created {@link Parent} and its controller.
     * @throws IOException If an IO error occurs.
     */
    public static <T> FXMLElement<T> load(String path) throws IOException {
        FXMLLoader loader = new FXMLLoader(FXMLFactory.class.getResource(path));

        return new FXMLElement<>(loader.load(), loader.getController());
    }

    /**
     * A simple record representing a loaded FXML document.
     *
     * @param parent     The {@link Parent} that was loaded.
     * @param controller The {@link Parent}'s controller.
     * @param <T>        The type of the controller.
     */
    public record FXMLElement<T>(Parent parent, T controller) {

    }

    private FXMLFactory() {
        throw new IllegalStateException("Utility class");
    }

}
