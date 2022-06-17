package net.insprill.customcarmanager.ui;

import javafx.application.Application;
import javafx.scene.Scene;
import javafx.scene.layout.BorderPane;
import javafx.stage.Stage;
import net.insprill.customcarmanager.config.Locale;

public class Window extends Application {

    private Stage primaryStage;

    @Override
    public void start(Stage primaryStage) {
        this.primaryStage = primaryStage;
        this.primaryStage.setTitle(Locale.getLine("window.title"));

        BorderPane pane = new BorderPane();

        Scene scene = new Scene(pane, 800, 600);

        this.primaryStage.setScene(scene);
        this.primaryStage.show();
    }

}
