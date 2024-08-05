#[doc = "Register `CRC_CTRL1` reader"]
pub type R = crate::R<CrcCtrl1Spec>;
#[doc = "Register `CRC_CTRL1` writer"]
pub type W = crate::W<CrcCtrl1Spec>;
#[doc = "Field `PWDN` reader - 0:0\\]
Power Down. When set, MCRC moduleMCRC Module is put in power down mode. 0 = MCRC is not in power down mode 1 = MCRC is in power down mode"]
pub type PwdnR = crate::BitReader;
#[doc = "Field `PWDN` writer - 0:0\\]
Power Down. When set, MCRC moduleMCRC Module is put in power down mode. 0 = MCRC is not in power down mode 1 = MCRC is in power down mode"]
pub type PwdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - 31:1\\]
Reserved"]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `Reserved1` writer - 31:1\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Power Down. When set, MCRC moduleMCRC Module is put in power down mode. 0 = MCRC is not in power down mode 1 = MCRC is in power down mode"]
    #[inline(always)]
    pub fn pwdn(&self) -> PwdnR {
        PwdnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Power Down. When set, MCRC moduleMCRC Module is put in power down mode. 0 = MCRC is not in power down mode 1 = MCRC is in power down mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwdn(&mut self) -> PwdnW<CrcCtrl1Spec> {
        PwdnW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<CrcCtrl1Spec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "Contains power down control bit\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcCtrl1Spec;
impl crate::RegisterSpec for CrcCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_ctrl1::R`](R) reader structure"]
impl crate::Readable for CrcCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`crc_ctrl1::W`](W) writer structure"]
impl crate::Writable for CrcCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_CTRL1 to value 0"]
impl crate::Resettable for CrcCtrl1Spec {
    const RESET_VALUE: u32 = 0;
}
