package net.insprill.customcarmanager.ui;

import javafx.application.Application;
import javafx.fxml.FXMLLoader;
import javafx.scene.Parent;
import javafx.scene.Scene;
import javafx.stage.Stage;
import net.insprill.customcarmanager.config.Locale;

import java.io.IOException;

public class Window extends Application {

    @Override
    public void start(Stage primaryStage) throws IOException {
        primaryStage.setTitle(Locale.getLine("window.title"));

        Parent root = FXMLLoader.load(getClass().getClassLoader().getResource("ui/home.fxml"));



        Scene scene = new Scene(root, 600, 400);

        primaryStage.setScene(scene);
        primaryStage.setResizable(false);
        primaryStage.show();
    }

}
