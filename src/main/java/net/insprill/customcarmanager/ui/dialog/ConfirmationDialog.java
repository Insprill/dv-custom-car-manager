package net.insprill.customcarmanager.ui.dialog;

import javafx.scene.control.Alert;
import javafx.scene.control.Button;
import javafx.scene.control.ButtonType;
import javafx.stage.StageStyle;

import java.util.Optional;

public class ConfirmationDialog extends Alert {

    private ConfirmationDialog(String message) {
        super(AlertType.CONFIRMATION, null, ButtonType.YES, ButtonType.NO);
        ((Button) getDialogPane().lookupButton(ButtonType.YES)).setDefaultButton(false);
        ((Button) getDialogPane().lookupButton(ButtonType.NO)).setDefaultButton(true);
        initStyle(StageStyle.UTILITY);
        headerTextProperty().setValue(null);
        setContentText(message);
    }

    public static boolean show(String message) {
        Optional<ButtonType> result = new ConfirmationDialog(message).showAndWait();
        return result.filter(buttonType -> buttonType == ButtonType.YES).isPresent();
    }

}
