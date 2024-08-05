#[doc = "Register `REVISION` reader"]
pub type R = crate::R<RevisionSpec>;
#[doc = "Register `REVISION` writer"]
pub type W = crate::W<RevisionSpec>;
#[doc = "Field `REV` reader - 7:0\\]
IP revision \\[7:4\\]
Major revision \\[3:0\\]
Minor revision Examples: 0x10 for 10 0x21 for 21 - (RO"]
pub type RevR = crate::FieldReader;
#[doc = "Field `REV` writer - 7:0\\]
IP revision \\[7:4\\]
Major revision \\[3:0\\]
Minor revision Examples: 0x10 for 10 0x21 for 21 - (RO"]
pub type RevW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED_13` reader - 31:8\\]
Reads returns 0 - (RO )"]
pub type Reserved13R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED_13` writer - 31:8\\]
Reads returns 0 - (RO )"]
pub type Reserved13W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
IP revision \\[7:4\\]
Major revision \\[3:0\\]
Minor revision Examples: 0x10 for 10 0x21 for 21 - (RO"]
    #[inline(always)]
    pub fn rev(&self) -> RevR {
        RevR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reads returns 0 - (RO )"]
    #[inline(always)]
    pub fn reserved_13(&self) -> Reserved13R {
        Reserved13R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
IP revision \\[7:4\\]
Major revision \\[3:0\\]
Minor revision Examples: 0x10 for 10 0x21 for 21 - (RO"]
    #[inline(always)]
    #[must_use]
    pub fn rev(&mut self) -> RevW<RevisionSpec> {
        RevW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reads returns 0 - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_13(&mut self) -> Reserved13W<RevisionSpec> {
        Reserved13W::new(self, 8)
    }
}
#[doc = "This register contains the hard coded RTL revision number.\n\nYou can [`read`](crate::Reg::read) this register and get [`revision::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`revision::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RevisionSpec;
impl crate::RegisterSpec for RevisionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`revision::R`](R) reader structure"]
impl crate::Readable for RevisionSpec {}
#[doc = "`write(|w| ..)` method takes [`revision::W`](W) writer structure"]
impl crate::Writable for RevisionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REVISION to value 0"]
impl crate::Resettable for RevisionSpec {
    const RESET_VALUE: u32 = 0;
}
