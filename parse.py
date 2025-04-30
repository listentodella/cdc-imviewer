import pandas as pd
import matplotlib.pyplot as plt
import numpy as np


odr = 400
# 读取数据
data = pd.read_csv('imu_data.csv', encoding='utf-8')
dps = data
# 计算标准差
gyro_std = data[['GX', 'GY', 'GZ']].std()
gyro_noise = gyro_std * (180/np.pi) / pow(odr/2, 0.5)
accel_std = data[['AX', 'AY', 'AZ']].std()
accel_noise = accel_std  / pow(odr/2, 0.5)
gyro_mean = data[['GX', 'GY', 'GZ']].mean()
accel_mean = data[['AX', 'AY', 'AZ']].mean()
temp_mean = data[['temperature']].mean()
# gyro_dps_std = data[['GX', 'GY', 'GZ']].multiply(np.pi/180).std()
# dps['GX'] = data['GX'] * (np.pi/180)
# dps['GY'] = data['GY'] * (np.pi/180)
# dps['GZ'] = data['GZ'] * (np.pi/180)
# gyro_dps_std = dps[['GX', 'GY', 'GZ']].std()

# 绘制陀螺仪数据变化曲线图
plt.figure(figsize=(12, 8))

plt.subplot(3, 2, 1)
plt.plot(data['frame'], data['GX'], label='Gyro X')
plt.plot(data['frame'], data['GY'], label='Gyro Y')
plt.plot(data['frame'], data['GZ'], label='Gyro Z')
plt.title('Gyro Data')
plt.xlabel('frame')
plt.ylabel('Gyro Reading')
# plt.legend()
plt.legend(loc='upper right')
plt.text(0.05, 0.95, f"Std Dev(rps): X={gyro_std['GX']:.5f}, Y={gyro_std['GY']:.5f}, Z={gyro_std['GZ']:.5f}", transform=plt.gca().transAxes)
plt.text(0.05, 0.80, f"Mean(rps): X={gyro_mean['GX']:.5f}, Y={gyro_mean['GY']:.5f}, Z={gyro_mean['GZ']:.5f}", transform=plt.gca().transAxes)
plt.text(0.05, 0.65, f"Noise(degree): X={gyro_noise['GX']:.5f}, Y={gyro_noise['GY']:.5f}, Z={gyro_noise['GZ']:.5f}", transform=plt.gca().transAxes)
print(f"Std Dev(rps): X={gyro_std['GX']:.5f}, Y={gyro_std['GY']:.5f}, Z={gyro_std['GZ']:.5f}")
print(f"Noise(degree): X={gyro_noise['GX']:.5f}, Y={gyro_noise['GY']:.5f}, Z={gyro_noise['GZ']:.5f}")
print(f"mean(rps): X={gyro_mean['GX']:.5f}, Y={gyro_mean['GY']:.5f}, Z={gyro_mean['GZ']:.5f}")

plt.subplot(3, 2, 3)
plt.plot(data['frame'], data['GX'], label='Gyro X')
plt.title('Gyro Data X')
plt.xlabel('frame')
plt.ylabel('Gyro X')
# plt.legend()
plt.legend(loc='upper right')

plt.subplot(3, 2, 4)
plt.plot(data['frame'], data['GY'], label='Gyro Y')
plt.title('Gyro Data Y')
plt.xlabel('frame')
plt.ylabel('Gyro Y')
# plt.legend()
plt.legend(loc='upper right')

plt.subplot(3, 2, 5)
plt.plot(data['frame'], data['GZ'], label='Gyro Z')
plt.title('Gyro Data Z')
plt.xlabel('frame')
plt.ylabel('Gyro Z')
# plt.legend()
plt.legend(loc='upper right')

plt.subplot(3, 2, 6)
plt.plot(data['frame'], data['temperature'], label='Temperature')
plt.title('Temperature Data')
plt.xlabel('frame')
plt.ylabel('temperature')
# plt.legend()
plt.legend(loc='upper right')


# 绘制加速度数据变化曲线图
plt.subplot(3, 2, 2)
plt.plot(data['frame'], data['AX'], label='Accel X')
plt.plot(data['frame'], data['AY'], label='Accel Y')
plt.plot(data['frame'], data['AZ'], label='Accel Z')
plt.title('Acceleration Data')
plt.xlabel('frame')
plt.ylabel('Acceleration Reading')
# plt.legend()
plt.legend(loc='upper right')
plt.text(0.05, 0.95, f"Std Dev: X={accel_std['AX']:.5f}, Y={accel_std['AY']:.5f}, Z={accel_std['AZ']:.5f}", transform=plt.gca().transAxes)
plt.text(0.05, 0.80, f"Mean: X={accel_mean['AX']:.5f}, Y={accel_mean['AY']:.5f}, Z={accel_mean['AZ']:.5f}", transform=plt.gca().transAxes)
plt.text(0.05, 0.65, f"Noise: X={accel_noise['AX']:.5f}, Y={accel_noise['AY']:.5f}, Z={accel_noise['AZ']:.5f}", transform=plt.gca().transAxes)
print(f"Std Dev: X={accel_std['AX']:.5f}, Y={accel_std['AY']:.5f}, Z={accel_std['AZ']:.5f}")
print(f"mean: X={accel_mean['AX']:.5f}, Y={accel_mean['AY']:.5f}, Z={accel_mean['AZ']:.5f}")
print(f"Noise: X={accel_noise['AX']:.5f}, Y={accel_noise['AY']:.5f}, Z={accel_noise['AZ']:.5f}")


# # 绘制加速度数据变化曲线图
# plt.subplot(3, 1, 2)
# plt.plot(data['frame'], data['AX'], label='Accel X')
# plt.plot(data['frame'], data['AY'], label='Accel Y')
# plt.plot(data['frame'], data['AZ'], label='Accel Z')
# plt.title('Acceleration Data')
# plt.xlabel('frame')
# plt.ylabel('Acceleration Reading')
# plt.legend()
# plt.text(0.05, 0.95, f"Std Dev: X={accel_std['AX']:.5f}, Y={accel_std['AY']:.5f}, Z={accel_std['AZ']:.5f}", transform=plt.gca().transAxes)
# plt.text(0.05, 0.80, f"Mean: X={accel_mean['AX']:.5f}, Y={accel_mean['AY']:.5f}, Z={accel_mean['AZ']:.5f}", transform=plt.gca().transAxes)
# print(f"Std Dev: X={accel_std['AX']:.5f}, Y={accel_std['AY']:.5f}, Z={accel_std['AZ']:.5f}")
# print(f"mean: X={accel_mean['AX']:.5f}, Y={accel_mean['AY']:.5f}, Z={accel_mean['AZ']:.5f}")

# 绘制温度数据变化曲线图
# plt.subplot(3, 1, 3)
# plt.plot(data['frame'], data['temperature'], label='Temperature')
# plt.title('Temperature Data')
# plt.xlabel('frame')
# plt.ylabel('Temperature Reading')
# plt.legend()

# 调整布局并显示图表
plt.tight_layout()
plt.show()
