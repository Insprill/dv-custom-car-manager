package net.insprill.customcarmanager.ui;

import javafx.application.Application;
import javafx.fxml.FXML;
import javafx.fxml.FXMLLoader;
import javafx.scene.Parent;
import javafx.scene.Scene;
import javafx.scene.control.TextField;
import javafx.stage.FileChooser;
import javafx.stage.Stage;
import net.insprill.customcarmanager.cars.CarManager;
import net.insprill.customcarmanager.config.Config;
import net.insprill.customcarmanager.config.Locale;
import net.insprill.customcarmanager.ui.factory.FileChooserFactory;
import net.insprill.customcarmanager.ui.factory.FolderChooserFactory;

import java.io.File;
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
    private CarManager carManager;

    @Override
    public void start(Stage primaryStage) throws IOException {
        Window.setInstance(this);

        this.primaryStage = primaryStage;
        this.carManager = new CarManager();

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

    public CarManager getCarManager() {
        return this.carManager;
    }

    // region Actions

    @FXML
    private void selectInstallDirectory() {
        File file = FolderChooserFactory.newDialog(Locale.getLine("folder-chooser.dv-install-directory.title"));
        if (file == null)
            return;
        String path = file.getAbsolutePath();
        Config.setString("install-directory", path);
        TextField lookup = (TextField) getPrimaryStage().getScene().lookup("#install_dir_field");
        lookup.setText(path);
    }

    @FXML
    private void installCarFromFolder() {
        File file = FolderChooserFactory.newDialog(Locale.getLine("folder-chooser.install-car.title"));
        if (file == null)
            return;
        getCarManager().installCarFromFolder(file);
    }

    @FXML
    private void installCarFromArchive() {
        File file = FileChooserFactory.newDialog(Locale.getLine("folder-chooser.install-car.title"), new FileChooser.ExtensionFilter("Archive", "*.zip"));
        if (file == null)
            return;
        getCarManager().installCarFromArchive(file);
    }

    // endregion

}
