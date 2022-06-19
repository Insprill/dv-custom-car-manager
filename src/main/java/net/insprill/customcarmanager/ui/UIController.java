package net.insprill.customcarmanager.ui;

import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.scene.Node;
import javafx.scene.control.TextField;
import javafx.scene.text.Text;
import javafx.stage.FileChooser;
import net.insprill.customcarmanager.cars.CarManager;
import net.insprill.customcarmanager.config.Config;
import net.insprill.customcarmanager.config.Locale;
import net.insprill.customcarmanager.ui.dialog.ErrorDialog;
import net.insprill.customcarmanager.ui.factory.FileChooserFactory;
import net.insprill.customcarmanager.ui.factory.FolderChooserFactory;

import java.io.File;
import java.util.Arrays;

public class UIController {

    @FXML
    private void selectInstallDirectory() {
        File file = FolderChooserFactory.newDialog(Locale.getLine("folder-chooser.dv-install-directory.title"));
        if (file == null)
            return;

        if (Arrays.stream(file.listFiles()).noneMatch(f -> f.getName().equals("DerailValley.exe"))) {
            new ErrorDialog(Locale.getLine("dialog.error.invalid-install-dir"));
            return;
        }

        if (!new File(file, CarManager.CARS_DIR).exists()) {
            new ErrorDialog(Locale.getLine("dialog.error.ccl-not-found"));
            return;
        }

        String path = file.getAbsolutePath();
        Config.setString("install-directory", path);
        TextField lookup = (TextField) Window.getInstance().findNode("#install_dir_field");
        lookup.setText(path);
        updateCars();
    }

    @FXML
    private void installCarFromFolder() {
        if (!checkInstallDir())
            return;
        File file = FolderChooserFactory.newDialog(Locale.getLine("folder-chooser.install-car.title"));
        if (file == null)
            return;
        Window.getInstance().getCarManager().installCarFromFolder(file);
        updateCars();
    }

    @FXML
    private void installCarFromArchive() {
        if (!checkInstallDir())
            return;
        File file = FileChooserFactory.newDialog(Locale.getLine("folder-chooser.install-car.title"), new FileChooser.ExtensionFilter("Zip files (*.zip)", "*.zip", "*.ZIP"));
        if (file == null)
            return;
        Window.getInstance().getCarManager().installCarFromArchive(file);
        updateCars();
    }

    @FXML
    public void deleteCar(ActionEvent value) {
        if (!checkInstallDir())
            return;
        String carName = ((Text) ((Node) value.getSource()).getParent().lookup("#car_name")).getText();
        Window.getInstance().getCarManager().getCar(carName).delete();
        updateCars();
    }

    public static void updateCars() {
        if (!checkInstallDir())
            return;
        Window.getInstance().getCarManager().populateCars();
        Window.getInstance().populateCarList();
    }

    private static boolean checkInstallDir() {
        if (Config.getString("install-directory").isEmpty()) {
            new ErrorDialog(Locale.getLine("dialog.error.no-install-dir"));
            return false;
        }
        return true;
    }

}
