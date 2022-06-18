package net.insprill.customcarmanager.ui.dialog;

import javafx.scene.control.Alert;
import javafx.scene.control.ButtonType;
import javafx.stage.Stage;
import javafx.stage.StageStyle;
import net.insprill.customcarmanager.config.Locale;

public class ErrorDialog extends Alert {

    public ErrorDialog(Stage primaryStage, String errorMessage) {
        super(AlertType.ERROR, null, ButtonType.CLOSE);
        init(primaryStage);
        setContentText(errorMessage);
        showAndWait();
    }

    private void init(Stage primaryStage) {
        initStyle(StageStyle.UTILITY);
        getDialogPane().getStyleClass().add("error-dialog-pane");
        headerTextProperty().setValue(Locale.getLine("dialog.error.title"));
        getDialogPane().getStylesheets().addAll(primaryStage.getScene().getStylesheets());
    }

}
