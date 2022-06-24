package net.insprill.customcarmanager.ui.factory;

import javafx.fxml.FXMLLoader;
import javafx.scene.Parent;

import java.io.IOException;

public class FXMLFactory {

    public static <T> FXMLElement<T> load(String path) throws IOException {
        FXMLLoader loader = new FXMLLoader(FXMLFactory.class.getResource(path));

        return new FXMLElement<>(loader.load(), loader.getController());
    }

    public record FXMLElement<T>(Parent parent, T controller) {

    }

}
