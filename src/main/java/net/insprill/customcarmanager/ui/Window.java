package net.insprill.customcarmanager.ui;

import javafx.application.Application;
import javafx.fxml.FXMLLoader;
import javafx.scene.Parent;
import javafx.scene.Scene;
import javafx.stage.Stage;
import net.insprill.customcarmanager.config.Locale;

import java.io.IOException;

public final class Window extends Application {

    // region Singleton
    private static Window instance;

    public static Window getInstance() {
        return instance;
    }

    private static void setInstance(Window instance) {
        Window.instance = instance;
    }
    // endregion

    private Stage primaryStage;

    @Override
    public void start(Stage primaryStage) throws IOException {
        Window.setInstance(this);

        this.primaryStage = primaryStage;

        primaryStage.setTitle(Locale.getLine("window.title"));

        Parent root = FXMLLoader.load(getClass().getClassLoader().getResource("ui/home.fxml"));



        Scene scene = new Scene(root, 600, 400);

        primaryStage.setScene(scene);
        primaryStage.setResizable(false);
        primaryStage.show();
    }

    public Stage getPrimaryStage() {
        return this.primaryStage;
    }

}
