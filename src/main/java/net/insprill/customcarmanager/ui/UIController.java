package net.insprill.customcarmanager.ui;

import javafx.application.Platform;
import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.scene.Node;
import javafx.scene.control.TextField;
import javafx.scene.input.DragEvent;
import javafx.scene.input.TransferMode;
import javafx.scene.text.Text;
import javafx.stage.FileChooser;
import net.insprill.customcarmanager.cars.CarManager;
import net.insprill.customcarmanager.config.Config;
import net.insprill.customcarmanager.config.Locale;
import net.insprill.customcarmanager.ui.dialog.ConfirmationDialog;
import net.insprill.customcarmanager.ui.dialog.ErrorDialog;
import net.insprill.customcarmanager.ui.dialog.InfoDialog;
import net.insprill.customcarmanager.ui.factory.FileChooserFactory;
import net.insprill.customcarmanager.ui.factory.FolderChooserFactory;

import java.io.File;
import java.util.Arrays;
import java.util.List;

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
    private void onDragOver(DragEvent event) {
        if (!event.getDragboard().hasFiles())
            return;

        event.acceptTransferModes(TransferMode.COPY_OR_MOVE);
        event.consume();
    }

    @FXML
    private void onDragDropped(DragEvent event) {
        if (!CarManager.checkInstallDir(true))
            return;
        if (!event.getDragboard().hasFiles())
            return;

        event.setDropCompleted(true);
        event.consume();

        List<File> files = event.getDragboard().getFiles();

        // If we don't run this later the drag/drop icon will stay until all dialogs are closed
        Platform.runLater(() -> {
            for (File file : files) {
                if (file.isDirectory()) {
                    Window.getInstance().getCarManager().installCarFromFolder(file);
                } else {
                    Window.getInstance().getCarManager().installCarFromArchive(file);
                }
            }
            updateCars();
        });
    }

    @FXML
    private void installCarFromFolder() {
        if (!CarManager.checkInstallDir(true))
            return;

        File file = FolderChooserFactory.newDialog(Locale.getLine("folder-chooser.install-car.title"));
        if (file == null)
            return;

        Window.getInstance().getCarManager().installCarFromFolder(file);
        updateCars();
    }

    @FXML
    private void installCarFromArchive() {
        if (!CarManager.checkInstallDir(true))
            return;

        File file = FileChooserFactory.newDialog(Locale.getLine("folder-chooser.install-car.title"), new FileChooser.ExtensionFilter("Archive", "*.zip", "*.ZIP", "*.rar", "*.RAR"));
        if (file == null)
            return;

        Window.getInstance().getCarManager().installCarFromArchive(file);
        updateCars();
    }

    @FXML
    public void deleteCar(ActionEvent value) {
        if (!CarManager.checkInstallDir(true))
            return;

        String carName = ((Text) ((Node) value.getSource()).getParent().lookup("#car_name")).getText();
        String confirmMsg = Locale.getLine("dialog.confirmation.delete-car").formatted(carName);
        if (!ConfirmationDialog.show(confirmMsg))
            return;

        Window.getInstance().getCarManager().getCar(carName).delete();
        updateCars();

        new InfoDialog(Locale.getLine("dialog.info.car-deleted").formatted(carName));
    }

    public static void updateCars() {
        Window.getInstance().getCarManager().populateCars();
        Window.getInstance().populateCarList();
    }

}
