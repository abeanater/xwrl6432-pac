#[doc = "Register `CRC_CURSEC_REG3` reader"]
pub type R = crate::R<CrcCursecReg3Spec>;
#[doc = "Register `CRC_CURSEC_REG3` writer"]
pub type W = crate::W<CrcCursecReg3Spec>;
#[doc = "Field `NU43` reader - 15:0\\]
Reserved"]
pub type Nu43R = crate::FieldReader<u16>;
#[doc = "Field `NU43` writer - 15:0\\]
Reserved"]
pub type Nu43W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `Reserved1` reader - 31:16\\]
Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` writer - 31:16\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu43(&self) -> Nu43R {
        Nu43R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu43(&mut self) -> Nu43W<CrcCursecReg3Spec> {
        Nu43W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<CrcCursecReg3Spec> {
        Reserved1W::new(self, 16)
    }
}
#[doc = "Channel 3 current sector register contains the sector number which causes CRC fail-ure\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_cursec_reg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_cursec_reg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcCursecReg3Spec;
impl crate::RegisterSpec for CrcCursecReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_cursec_reg3::R`](R) reader structure"]
impl crate::Readable for CrcCursecReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`crc_cursec_reg3::W`](W) writer structure"]
impl crate::Writable for CrcCursecReg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_CURSEC_REG3 to value 0"]
impl crate::Resettable for CrcCursecReg3Spec {
    const RESET_VALUE: u32 = 0;
}
