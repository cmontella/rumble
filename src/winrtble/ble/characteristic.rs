use ::Result;
use ::Error;
use winrt::ComPtr;
use winrt::windows::devices::bluetooth::genericattributeprofile::{GattReadResult, GattCommunicationStatus, GattCharacteristic, GattValueChangedEventArgs, GattClientCharacteristicConfigurationDescriptorValue};
use winrt::windows::storage::streams::DataReader;
use winrt::RtAsyncOperation;
use winrt::windows::foundation::{ TypedEventHandler, EventRegistrationToken };
use winrt::windows::foundation::{AsyncOperationCompletedHandler, IAsyncOperation, AsyncStatus};
use api::RequestCallback;
use winrt::Result as WinRTResult;

pub type NotifiyEventHandler = Box<Fn(Vec<u8>) + Send>;

pub struct BLECharacteristic {
    characteristic: ComPtr<GattCharacteristic>,
    notify_token: Option<EventRegistrationToken>,
}

unsafe impl Send for BLECharacteristic {}
unsafe impl Sync for BLECharacteristic {}

/*impl<T> Into<Result<T>> for WinRTResult<T> {
    fn into(self) -> Result<T> {
        Err(Error::DeviceNotFound)
    }
}*/

impl BLECharacteristic {
    pub fn new(characteristic: ComPtr<GattCharacteristic>) -> Self {
        BLECharacteristic { characteristic, notify_token: None }
    }

    fn value_from_read_result(result: &ComPtr<GattReadResult>) -> Result<Vec<u8>> {
        if result.get_status()? == GattCommunicationStatus::Success {
            let value = result.get_value()?.unwrap();
            let reader = DataReader::from_buffer(&value)?.unwrap();
            let len = reader.get_unconsumed_buffer_length()? as usize;
            let mut input = vec![0u8; len];
            reader.read_bytes(&mut input[0..len])?;
            Ok(input)
        } else {
            Err(Error::NotSupported("get_status".into()))
        }
    }

    pub fn read_value(&self) -> Result<Vec<u8>> {
        let result = self.characteristic.read_value_async()?.blocking_get()?.unwrap();
        Self::value_from_read_result(&result)
    }

    pub fn read_value_async(&self, callback: Option<RequestCallback>) {
        match self.characteristic.read_value_async() {
            Ok(async_op) => {
                let handler = AsyncOperationCompletedHandler::new(move |op, status| {
                    let result = if status == AsyncStatus::Completed {
                        let async_op : &IAsyncOperation<GattReadResult> = unsafe { (&*op) };
                        let result = async_op.get_results().unwrap().unwrap();
                        Self::value_from_read_result(&result)
                    } else {
                        Err(Error::Other(format!("AsyncStatus {:?}", status)))
                    };
                    if let Some(ref callback) = callback {
                        callback(result);
                    }
                    Ok(())
                });
                async_op.set_completed(&handler).expect("set_completed failed");
            },
            Err(e) => {
                if let Some(ref callback) = callback {
                    callback(Err(e.into()));
                }
            }
        }
    }

    pub fn subscribe(&mut self, on_value_changed: NotifiyEventHandler) -> Result<()> {
        let value_handler = TypedEventHandler::new(move |_: *mut GattCharacteristic, args: *mut GattValueChangedEventArgs| {
            let args = unsafe { (&*args) };
            let value = args.get_characteristic_value().unwrap().unwrap();
            let reader = DataReader::from_buffer(&value).unwrap().unwrap();
            let len = reader.get_unconsumed_buffer_length().unwrap() as usize;
            let mut input = vec![0u8; len];
            reader.read_bytes(&mut input[0..len]).unwrap();
            println!("changed {:?}", input);
            on_value_changed(input);
            Ok(())
        });
        let token = self.characteristic.add_value_changed(&value_handler).unwrap();
        self.notify_token = Some(token);
        let config = GattClientCharacteristicConfigurationDescriptorValue::Indicate;
        let status = self.characteristic.write_client_characteristic_configuration_descriptor_async(config).unwrap().blocking_get().unwrap();
        println!("{:?}", status);
        Ok(())
    }

    pub fn unsubscribe(&mut self) -> Result<()> {
        if let Some(token) = self.notify_token {
            self.characteristic.remove_value_changed(token).unwrap();
        }
        self.notify_token = None;
        let config = GattClientCharacteristicConfigurationDescriptorValue::None;
        let status = self.characteristic.write_client_characteristic_configuration_descriptor_async(config).unwrap().blocking_get().unwrap();
        println!("{:?}", status);
        Ok(())
    }
}

impl Drop for BLECharacteristic {
    fn drop(&mut self) {
        if let Some(token) = self.notify_token {
            let result = self.characteristic.remove_value_changed(token);
            if let Err(err) = result {
                println!("Drop:remove_connection_status_changed {:?}", err);
            }
        }
    }
}
