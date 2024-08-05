#[doc = "Register `ICPDRV` reader"]
pub type R = crate::R<IcpdrvSpec>;
#[doc = "Register `ICPDRV` writer"]
pub type W = crate::W<IcpdrvSpec>;
#[doc = "Field `PDRV0` reader - 0:0\\]
Used to select driver mode of output buffer for SCL pin. 0 = I2C mode. 1 = GPIO mode. Note: Value of this register is reflected on the PDRV_SCL_POR port. Actual function depends on I/O buffer and chip implementation."]
pub type Pdrv0R = crate::BitReader;
#[doc = "Field `PDRV0` writer - 0:0\\]
Used to select driver mode of output buffer for SCL pin. 0 = I2C mode. 1 = GPIO mode. Note: Value of this register is reflected on the PDRV_SCL_POR port. Actual function depends on I/O buffer and chip implementation."]
pub type Pdrv0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDRV1` reader - 1:1\\]
Used to select driver mode of output buffer for SDA pin. 0 = I2C mode. 1 = GPIO mode. Note: Value of this register is reflected on the PDRV_SDA_POR port. Actual function depends on I/O buffer and chip implementation."]
pub type Pdrv1R = crate::BitReader;
#[doc = "Field `PDRV1` writer - 1:1\\]
Used to select driver mode of output buffer for SDA pin. 0 = I2C mode. 1 = GPIO mode. Note: Value of this register is reflected on the PDRV_SDA_POR port. Actual function depends on I/O buffer and chip implementation."]
pub type Pdrv1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 31:2\\]
Reserved"]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:2\\]
Reserved"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Used to select driver mode of output buffer for SCL pin. 0 = I2C mode. 1 = GPIO mode. Note: Value of this register is reflected on the PDRV_SCL_POR port. Actual function depends on I/O buffer and chip implementation."]
    #[inline(always)]
    pub fn pdrv0(&self) -> Pdrv0R {
        Pdrv0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Used to select driver mode of output buffer for SDA pin. 0 = I2C mode. 1 = GPIO mode. Note: Value of this register is reflected on the PDRV_SDA_POR port. Actual function depends on I/O buffer and chip implementation."]
    #[inline(always)]
    pub fn pdrv1(&self) -> Pdrv1R {
        Pdrv1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Used to select driver mode of output buffer for SCL pin. 0 = I2C mode. 1 = GPIO mode. Note: Value of this register is reflected on the PDRV_SCL_POR port. Actual function depends on I/O buffer and chip implementation."]
    #[inline(always)]
    #[must_use]
    pub fn pdrv0(&mut self) -> Pdrv0W<IcpdrvSpec> {
        Pdrv0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Used to select driver mode of output buffer for SDA pin. 0 = I2C mode. 1 = GPIO mode. Note: Value of this register is reflected on the PDRV_SDA_POR port. Actual function depends on I/O buffer and chip implementation."]
    #[inline(always)]
    #[must_use]
    pub fn pdrv1(&mut self) -> Pdrv1W<IcpdrvSpec> {
        Pdrv1W::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<IcpdrvSpec> {
        NuW::new(self, 2)
    }
}
#[doc = "I2C Pin Driver Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icpdrv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpdrv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcpdrvSpec;
impl crate::RegisterSpec for IcpdrvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icpdrv::R`](R) reader structure"]
impl crate::Readable for IcpdrvSpec {}
#[doc = "`write(|w| ..)` method takes [`icpdrv::W`](W) writer structure"]
impl crate::Writable for IcpdrvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICPDRV to value 0"]
impl crate::Resettable for IcpdrvSpec {
    const RESET_VALUE: u32 = 0;
}
