#[doc = "Register `ICPID1` reader"]
pub type R = crate::R<Icpid1Spec>;
#[doc = "Register `ICPID1` writer"]
pub type W = crate::W<Icpid1Spec>;
#[doc = "Field `REVISION` reader - 7:0\\]
Identifies the revision level of the I2C. This value should be incremented each time the design is revised. - (RW )"]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `REVISION` writer - 7:0\\]
Identifies the revision level of the I2C. This value should be incremented each time the design is revised. - (RW )"]
pub type RevisionW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLASS` reader - 15:8\\]
Identifies the class of peripheral. This value should be 0x01 - (RW )"]
pub type ClassR = crate::FieldReader;
#[doc = "Field `CLASS` writer - 15:8\\]
Identifies the class of peripheral. This value should be 0x01 - (RW )"]
pub type ClassW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU` reader - 31:16\\]
Reserved."]
pub type NuR = crate::FieldReader<u16>;
#[doc = "Field `NU` writer - 31:16\\]
Reserved."]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Identifies the revision level of the I2C. This value should be incremented each time the design is revised. - (RW )"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Identifies the class of peripheral. This value should be 0x01 - (RW )"]
    #[inline(always)]
    pub fn class(&self) -> ClassR {
        ClassR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved."]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Identifies the revision level of the I2C. This value should be incremented each time the design is revised. - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn revision(&mut self) -> RevisionW<Icpid1Spec> {
        RevisionW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Identifies the class of peripheral. This value should be 0x01 - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn class(&mut self) -> ClassW<Icpid1Spec> {
        ClassW::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Icpid1Spec> {
        NuW::new(self, 16)
    }
}
#[doc = "I2C Peripheral ID register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`icpid1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpid1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Icpid1Spec;
impl crate::RegisterSpec for Icpid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icpid1::R`](R) reader structure"]
impl crate::Readable for Icpid1Spec {}
#[doc = "`write(|w| ..)` method takes [`icpid1::W`](W) writer structure"]
impl crate::Writable for Icpid1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICPID1 to value 0"]
impl crate::Resettable for Icpid1Spec {
    const RESET_VALUE: u32 = 0;
}
